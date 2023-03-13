#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vocabulary {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / 单词
    #[prost(string, tag = "2")]
    pub word: ::prost::alloc::string::String,
    /// / 音标
    #[prost(string, tag = "3")]
    pub soundmark: ::prost::alloc::string::String,
    /// / 词根
    #[prost(string, tag = "4")]
    pub roots: ::prost::alloc::string::String,
    /// / 释义
    #[prost(string, tag = "5")]
    pub paraphrase: ::prost::alloc::string::String,
    /// / 词组
    #[prost(string, tag = "6")]
    pub collocations: ::prost::alloc::string::String,
    /// / 同义词
    #[prost(string, tag = "7")]
    pub synonyms: ::prost::alloc::string::String,
    /// / 例句
    #[prost(string, tag = "8")]
    pub examples: ::prost::alloc::string::String,
    /// / 创建时间
    #[prost(message, optional, tag = "9")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / 更新时间
    #[prost(message, optional, tag = "10")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
