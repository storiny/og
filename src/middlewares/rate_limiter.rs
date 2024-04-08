use actix_extensible_rate_limit::backend::{
    Backend,
    SimpleBackend,
    SimpleInput,
    SimpleOutput,
};
use actix_web::{
    HttpResponse,
    ResponseError,
};
use async_trait::async_trait;
use redis::{
    aio::ConnectionManager,
    AsyncCommands,
};
use std::{
    borrow::Cow,
    time::Duration,
};
use thiserror::Error;
use tokio::time::Instant;

/// Macro for executing a Redis transaction in an asynchronous context.
///
/// * `$conn` - Redis connection that implements the `Connection` trait.
/// * `$keys` - Array of keys to watch during the transaction.
/// * `$body` - Body of the transaction to be executed.
macro_rules! async_transaction {
    ($conn:expr, $keys:expr, $body:expr) => {
        loop {
            redis::cmd("WATCH").arg($keys).query_async($conn).await?;

            if let Some(response) = $body {
                redis::cmd("UNWATCH").query_async($conn).await?;
                break response;
            }
        }
    };
}

/// Custom error type for Redis-related errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Redis error
    #[error("Redis error: {0}")]
    Redis(
        #[source]
        #[from]
        redis::RedisError,
    ),
    /// Unexpected negative TTL response from Redis
    #[error("Unexpected negative TTL response")]
    NegativeTtl,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().finish()
    }
}

/// A Fixed Window rate limiter [Backend] that stores data in Redis.
#[derive(Clone)]
pub struct RedisBackend {
    connection: ConnectionManager,
    key_prefix: Option<String>,
}

/// Builder for configuring a `RedisBackend`.
pub struct Builder {
    connection: ConnectionManager,
    key_prefix: Option<String>,
}

impl RedisBackend {
    /// Creates a `RedisBackend` builder.
    ///
    /// * `connection` - A Redis connection pool
    pub fn builder(connection: ConnectionManager) -> Builder {
        Builder {
            connection,
            key_prefix: None,
        }
    }

    fn make_key<'t>(&self, key: &'t str) -> Cow<'t, str> {
        match &self.key_prefix {
            None => Cow::Borrowed(key),
            Some(prefix) => Cow::Owned(format!("{}{}", prefix, key)),
        }
    }
}

impl Builder {
    /// Applies an optional prefix to all rate limit keys given to this backend (prevents collision
    /// across services).
    pub fn key_prefix(mut self, key_prefix: Option<&str>) -> Self {
        self.key_prefix = key_prefix.map(ToOwned::to_owned);
        self
    }

    /// Builds the `RedisBackend` instance with the configured options.
    pub fn build(self) -> RedisBackend {
        RedisBackend {
            connection: self.connection,
            key_prefix: self.key_prefix,
        }
    }
}

#[async_trait(?Send)]
impl Backend<SimpleInput> for RedisBackend {
    type Output = SimpleOutput;
    type RollbackToken = String;
    type Error = Error;

    /// Requests rate limiting
    async fn request(
        &self,
        input: SimpleInput,
    ) -> Result<(bool, Self::Output, Self::RollbackToken), Self::Error> {
        let key = self.make_key(&input.key);
        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("SET")
            .arg(key.as_ref())
            .arg(0_i64)
            .arg("EX")
            .arg(input.interval.as_secs())
            .arg("NX")
            .ignore()
            .cmd("INCR")
            .arg(key.as_ref())
            .cmd("TTL")
            .arg(key.as_ref());

        let mut con = self.connection.clone();
        let (count, ttl): (u64, i64) = pipe.query_async(&mut con).await?;

        if ttl < 0 {
            return Err(Error::NegativeTtl);
        }

        let allow = count <= input.max_requests;
        let output = SimpleOutput {
            limit: input.max_requests,
            remaining: input.max_requests.saturating_sub(count),
            reset: Instant::now() + Duration::from_secs(ttl as u64),
        };

        Ok((allow, output, input.key))
    }

    /// Rolls back the rate limiting operation
    async fn rollback(&self, token: Self::RollbackToken) -> Result<(), Self::Error> {
        let key = self.make_key(&token);
        let mut con = self.connection.clone();

        async_transaction!(&mut con, &[key.as_ref()], {
            let old_val: Option<u64> = con.get(key.as_ref()).await?;
            if let Some(old_val) = old_val {
                if old_val >= 1 {
                    redis::pipe()
                        .atomic()
                        .decr::<_, u64>(key.as_ref(), 1)
                        .ignore()
                        .query_async::<_, Option<()>>(&mut con)
                        .await?
                } else {
                    Some(())
                }
            } else {
                Some(())
            }
        });

        Ok(())
    }
}

#[async_trait(?Send)]
impl SimpleBackend for RedisBackend {
    /// Removes a key from the Redis backend store
    async fn remove_key(&self, key: &str) -> Result<(), Self::Error> {
        let key = self.make_key(key);
        let mut con = self.connection.clone();
        con.del(key.as_ref()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_extensible_rate_limit::HeaderCompatibleOutput;

    const MINUTE: Duration = Duration::from_secs(60);

    /// Creates a `RedisBackend` builder for testing.
    ///
    /// * `clear_test_key` - Key to be cleared before each test run.
    async fn make_backend(clear_test_key: &str) -> Builder {
        // Env
        let host = option_env!("REDIS_HOST").unwrap_or("127.0.0.1");
        let port = option_env!("REDIS_PORT").unwrap_or("6379");

        let client = redis::Client::open(format!("redis://{host}:{port}")).unwrap();
        let mut manager = ConnectionManager::new(client).await.unwrap();

        manager.del::<_, ()>(clear_test_key).await.unwrap();
        RedisBackend::builder(manager)
    }

    #[tokio::test]
    async fn can_allow_deny() {
        let backend = make_backend("test_allow_deny").await.build();
        let input = SimpleInput {
            interval: MINUTE,
            max_requests: 5,
            key: "test_allow_deny".to_string(),
        };
        for _ in 0..5 {
            // First 5 should be allowed
            let (allow, _, _) = backend.request(input.clone()).await.unwrap();
            assert!(allow);
        }
        // Sixth should be denied
        let (allow, _, _) = backend.request(input.clone()).await.unwrap();
        assert!(!allow);
    }

    #[tokio::test]
    async fn can_reset() {
        let backend = make_backend("test_reset").await.build();
        let input = SimpleInput {
            interval: Duration::from_secs(3),
            max_requests: 1,
            key: "test_reset".to_string(),
        };
        // Make first request, should be allowed
        let (allow, _, _) = backend.request(input.clone()).await.unwrap();
        assert!(allow);
        // Request again, should be denied
        let (allow, out, _) = backend.request(input.clone()).await.unwrap();
        assert!(!allow);
        // Sleep until reset, should now be allowed
        tokio::time::sleep(Duration::from_secs(out.seconds_until_reset())).await;
        let (allow, _, _) = backend.request(input).await.unwrap();
        assert!(allow);
    }

    #[tokio::test]
    async fn can_produce_output() {
        let backend = make_backend("test_output").await.build();
        let input = SimpleInput {
            interval: MINUTE,
            max_requests: 2,
            key: "test_output".to_string(),
        };
        // First of 2 should be allowed.
        let (allow, output, _) = backend.request(input.clone()).await.unwrap();
        assert!(allow);
        assert_eq!(output.remaining, 1);
        assert_eq!(output.limit, 2);
        assert!(output.seconds_until_reset() > 0 && output.seconds_until_reset() <= 60);
        // Second of 2 should be allowed.
        let (allow, output, _) = backend.request(input.clone()).await.unwrap();
        assert!(allow);
        assert_eq!(output.remaining, 0);
        assert_eq!(output.limit, 2);
        assert!(output.seconds_until_reset() > 0 && output.seconds_until_reset() <= 60);
        // Should be denied
        let (allow, output, _) = backend.request(input).await.unwrap();
        assert!(!allow);
        assert_eq!(output.remaining, 0);
        assert_eq!(output.limit, 2);
        assert!(output.seconds_until_reset() > 0 && output.seconds_until_reset() <= 60);
    }

    #[tokio::test]
    async fn can_rollback() {
        let backend = make_backend("test_rollback").await.build();
        let input = SimpleInput {
            interval: MINUTE,
            max_requests: 5,
            key: "test_rollback".to_string(),
        };
        let (_, output, rollback) = backend.request(input.clone()).await.unwrap();
        assert_eq!(output.remaining, 4);
        backend.rollback(rollback).await.unwrap();
        // Remaining requests should still be the same, since the previous call was excluded
        let (_, output, _) = backend.request(input).await.unwrap();
        assert_eq!(output.remaining, 4);
        // Check ttl is not corrupted
        assert!(output.seconds_until_reset() > 0 && output.seconds_until_reset() <= 60);
    }

    #[tokio::test]
    async fn can_rollback_key_gone() {
        let backend = make_backend("test_rollback_key_gone").await.build();
        let mut con = backend.connection.clone();
        // The rollback could happen after the key has already expired
        backend
            .rollback("test_rollback_key_gone".to_string())
            .await
            .unwrap();
        // In which case nothing should happen
        assert!(
            !con.exists::<_, bool>("test_rollback_key_gone")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn can_remove_key() {
        let backend = make_backend("test_remove_key").await.build();
        let input = SimpleInput {
            interval: MINUTE,
            max_requests: 1,
            key: "test_remove_key".to_string(),
        };
        let (allow, _, _) = backend.request(input.clone()).await.unwrap();
        assert!(allow);
        let (allow, _, _) = backend.request(input.clone()).await.unwrap();
        assert!(!allow);
        backend.remove_key("test_remove_key").await.unwrap();
        // Counter should have been reset
        let (allow, _, _) = backend.request(input).await.unwrap();
        assert!(allow);
    }

    #[tokio::test]
    async fn can_add_key_prefix() {
        let backend = make_backend("prefix:test_key_prefix")
            .await
            .key_prefix(Some("prefix:"))
            .build();
        let mut con = backend.connection.clone();
        let input = SimpleInput {
            interval: MINUTE,
            max_requests: 5,
            key: "test_key_prefix".to_string(),
        };
        backend.request(input.clone()).await.unwrap();
        assert!(
            con.exists::<_, bool>("prefix:test_key_prefix")
                .await
                .unwrap()
        );

        backend.remove_key("test_key_prefix").await.unwrap();
        assert!(
            !con.exists::<_, bool>("prefix:test_key_prefix")
                .await
                .unwrap()
        );
    }
}
