// @generated
impl serde::Serialize for GetStoryOpenGraphDataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("open_graph_def.v1.GetStoryOpenGraphDataRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStoryOpenGraphDataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStoryOpenGraphDataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct open_graph_def.v1.GetStoryOpenGraphDataRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetStoryOpenGraphDataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetStoryOpenGraphDataRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("open_graph_def.v1.GetStoryOpenGraphDataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStoryOpenGraphDataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.splash_id.is_some() {
            len += 1;
        }
        if self.like_count != 0 {
            len += 1;
        }
        if self.read_count != 0 {
            len += 1;
        }
        if self.comment_count != 0 {
            len += 1;
        }
        if self.is_private {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        if self.user_avatar_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("open_graph_def.v1.GetStoryOpenGraphDataResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.splash_id.as_ref() {
            struct_ser.serialize_field("splashId", v)?;
        }
        if self.like_count != 0 {
            struct_ser.serialize_field("likeCount", &self.like_count)?;
        }
        if self.read_count != 0 {
            struct_ser.serialize_field("readCount", &self.read_count)?;
        }
        if self.comment_count != 0 {
            struct_ser.serialize_field("commentCount", &self.comment_count)?;
        }
        if self.is_private {
            struct_ser.serialize_field("isPrivate", &self.is_private)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if let Some(v) = self.user_avatar_id.as_ref() {
            struct_ser.serialize_field("userAvatarId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStoryOpenGraphDataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "title",
            "description",
            "splash_id",
            "splashId",
            "like_count",
            "likeCount",
            "read_count",
            "readCount",
            "comment_count",
            "commentCount",
            "is_private",
            "isPrivate",
            "user_name",
            "userName",
            "user_avatar_id",
            "userAvatarId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Title,
            Description,
            SplashId,
            LikeCount,
            ReadCount,
            CommentCount,
            IsPrivate,
            UserName,
            UserAvatarId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "splashId" | "splash_id" => Ok(GeneratedField::SplashId),
                            "likeCount" | "like_count" => Ok(GeneratedField::LikeCount),
                            "readCount" | "read_count" => Ok(GeneratedField::ReadCount),
                            "commentCount" | "comment_count" => Ok(GeneratedField::CommentCount),
                            "isPrivate" | "is_private" => Ok(GeneratedField::IsPrivate),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "userAvatarId" | "user_avatar_id" => Ok(GeneratedField::UserAvatarId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStoryOpenGraphDataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct open_graph_def.v1.GetStoryOpenGraphDataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetStoryOpenGraphDataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut title__ = None;
                let mut description__ = None;
                let mut splash_id__ = None;
                let mut like_count__ = None;
                let mut read_count__ = None;
                let mut comment_count__ = None;
                let mut is_private__ = None;
                let mut user_name__ = None;
                let mut user_avatar_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::SplashId => {
                            if splash_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splashId"));
                            }
                            splash_id__ = map.next_value()?;
                        }
                        GeneratedField::LikeCount => {
                            if like_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("likeCount"));
                            }
                            like_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReadCount => {
                            if read_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readCount"));
                            }
                            read_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CommentCount => {
                            if comment_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commentCount"));
                            }
                            comment_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsPrivate => {
                            if is_private__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPrivate"));
                            }
                            is_private__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserAvatarId => {
                            if user_avatar_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAvatarId"));
                            }
                            user_avatar_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetStoryOpenGraphDataResponse {
                    id: id__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    description: description__,
                    splash_id: splash_id__,
                    like_count: like_count__.unwrap_or_default(),
                    read_count: read_count__.unwrap_or_default(),
                    comment_count: comment_count__.unwrap_or_default(),
                    is_private: is_private__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    user_avatar_id: user_avatar_id__,
                })
            }
        }
        deserializer.deserialize_struct("open_graph_def.v1.GetStoryOpenGraphDataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTagOpenGraphDataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("open_graph_def.v1.GetTagOpenGraphDataRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTagOpenGraphDataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTagOpenGraphDataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct open_graph_def.v1.GetTagOpenGraphDataRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTagOpenGraphDataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetTagOpenGraphDataRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("open_graph_def.v1.GetTagOpenGraphDataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTagOpenGraphDataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.story_count != 0 {
            len += 1;
        }
        if self.follower_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("open_graph_def.v1.GetTagOpenGraphDataResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.story_count != 0 {
            struct_ser.serialize_field("storyCount", &self.story_count)?;
        }
        if self.follower_count != 0 {
            struct_ser.serialize_field("followerCount", &self.follower_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTagOpenGraphDataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "story_count",
            "storyCount",
            "follower_count",
            "followerCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            StoryCount,
            FollowerCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "storyCount" | "story_count" => Ok(GeneratedField::StoryCount),
                            "followerCount" | "follower_count" => Ok(GeneratedField::FollowerCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTagOpenGraphDataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct open_graph_def.v1.GetTagOpenGraphDataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTagOpenGraphDataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut story_count__ = None;
                let mut follower_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::StoryCount => {
                            if story_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storyCount"));
                            }
                            story_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FollowerCount => {
                            if follower_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("followerCount"));
                            }
                            follower_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetTagOpenGraphDataResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    story_count: story_count__.unwrap_or_default(),
                    follower_count: follower_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("open_graph_def.v1.GetTagOpenGraphDataResponse", FIELDS, GeneratedVisitor)
    }
}
