// @generated
// Get story open graph data request

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryOpenGraphDataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoryOpenGraphDataResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub splash_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag="5")]
    pub like_count: u32,
    #[prost(uint32, tag="6")]
    pub read_count: u32,
    #[prost(uint32, tag="7")]
    pub comment_count: u32,
    #[prost(bool, tag="8")]
    pub is_private: bool,
    /// User
    #[prost(string, tag="9")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="10")]
    pub user_avatar_id: ::core::option::Option<::prost::alloc::string::String>,
}
// Get tag open graph data request

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagOpenGraphDataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagOpenGraphDataResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub story_count: u32,
    #[prost(uint32, tag="4")]
    pub follower_count: u32,
}
// @@protoc_insertion_point(module)
