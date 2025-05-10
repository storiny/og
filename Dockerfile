# Start with a rust alpine image
FROM rust:1.86-alpine3.21 AS builder

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# If needed, add additional dependencies here
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

# Set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app

# Do a release build
RUN cargo build --release
RUN strip target/release/storiny_og

# Use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.21

# If needed, install additional dependencies here
RUN apk add --no-cache libgcc

# Copy the binary into the final image
COPY --from=builder /app/target/release/storiny_og .

# copy static folder
COPY --from=builder /app/static /static

# Set the binary as entrypoint
ENTRYPOINT ["/storiny_og"]