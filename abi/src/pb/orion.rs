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
#[derive(to_sql_condition::ToSqlCondition, derive_builder::Builder)]
#[builder(setter(into), default)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / 单词
    #[prost(string, optional, tag = "2")]
    pub word: ::core::option::Option<::prost::alloc::string::String>,
    /// / 音标
    #[prost(string, optional, tag = "3")]
    pub soundmark: ::core::option::Option<::prost::alloc::string::String>,
    /// / 词根
    #[prost(string, optional, tag = "4")]
    pub roots: ::core::option::Option<::prost::alloc::string::String>,
    /// / 释义
    #[prost(string, optional, tag = "5")]
    pub paraphrase: ::core::option::Option<::prost::alloc::string::String>,
    /// / 词组
    #[prost(string, optional, tag = "6")]
    pub collocations: ::core::option::Option<::prost::alloc::string::String>,
    /// / 同义词
    #[prost(string, optional, tag = "7")]
    pub synonyms: ::core::option::Option<::prost::alloc::string::String>,
    /// / 例句
    #[prost(string, optional, tag = "8")]
    pub examples: ::core::option::Option<::prost::alloc::string::String>,
    /// / offset
    #[prost(int64, optional, tag = "9")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "10")]
    pub limit: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Story {
    /// / id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// / words
    #[prost(string, repeated, tag = "2")]
    pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// / content
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    /// / read count
    #[prost(int64, tag = "4")]
    pub read_count: i64,
    /// / created at
    #[prost(message, optional, tag = "5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// / updated at
    #[prost(message, optional, tag = "6")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryQuery {
    /// / id
    #[prost(int64, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// / words
    #[prost(string, optional, tag = "2")]
    pub words: ::core::option::Option<::prost::alloc::string::String>,
    /// / content
    #[prost(string, optional, tag = "3")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    /// / read count
    #[prost(int64, optional, tag = "4")]
    pub read_count: ::core::option::Option<i64>,
    /// / offset
    #[prost(int64, optional, tag = "5")]
    pub offset: ::core::option::Option<i64>,
    /// / limit
    #[prost(int64, optional, tag = "6")]
    pub limit: ::core::option::Option<i64>,
}
/// add a new vocabulary
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddVocabularyRequest {
    #[prost(message, optional, tag = "1")]
    pub vocabulary: ::core::option::Option<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VocabularyResponse {
    #[prost(message, optional, tag = "1")]
    pub vocabulary: ::core::option::Option<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<VocabularyQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyResponse {
    #[prost(message, repeated, tag = "1")]
    pub vocabulary: ::prost::alloc::vec::Vec<Vocabulary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVocabularyRandomRequest {
    #[prost(int64, tag = "1")]
    pub limit: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStoryRequest {
    #[prost(message, optional, tag = "1")]
    pub story: ::core::option::Option<Story>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoryResponse {
    #[prost(message, optional, tag = "1")]
    pub story: ::core::option::Option<Story>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStoryResponse {
    #[prost(message, repeated, tag = "1")]
    pub story: ::prost::alloc::vec::Vec<Story>,
}
/// Generated client implementations.
pub mod vocabulary_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// The Vocabulary service definition.
    #[derive(Debug, Clone)]
    pub struct VocabularyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VocabularyServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VocabularyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VocabularyServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            VocabularyServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// add a new vocabulary
        pub async fn add_vocabulary(
            &mut self,
            request: impl tonic::IntoRequest<super::AddVocabularyRequest>,
        ) -> Result<tonic::Response<super::VocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/orion.VocabularyService/AddVocabulary");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query vocabulary
        pub async fn query_vocabulary(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVocabularyRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/orion.VocabularyService/QueryVocabulary");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query random vocabulary
        pub async fn query_vocabulary_random(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryVocabularyRandomRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orion.VocabularyService/QueryVocabularyRandom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod story_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StoryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StoryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StoryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StoryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// add a new story
        pub async fn add_story(
            &mut self,
            request: impl tonic::IntoRequest<super::AddStoryRequest>,
        ) -> Result<tonic::Response<super::StoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/orion.StoryService/AddStory");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vocabulary_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VocabularyServiceServer.
    #[async_trait]
    pub trait VocabularyService: Send + Sync + 'static {
        /// add a new vocabulary
        async fn add_vocabulary(
            &self,
            request: tonic::Request<super::AddVocabularyRequest>,
        ) -> Result<tonic::Response<super::VocabularyResponse>, tonic::Status>;
        /// query vocabulary
        async fn query_vocabulary(
            &self,
            request: tonic::Request<super::QueryVocabularyRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status>;
        /// query random vocabulary
        async fn query_vocabulary_random(
            &self,
            request: tonic::Request<super::QueryVocabularyRandomRequest>,
        ) -> Result<tonic::Response<super::QueryVocabularyResponse>, tonic::Status>;
    }
    /// The Vocabulary service definition.
    #[derive(Debug)]
    pub struct VocabularyServiceServer<T: VocabularyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VocabularyService> VocabularyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VocabularyServiceServer<T>
    where
        T: VocabularyService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.VocabularyService/AddVocabulary" => {
                    #[allow(non_camel_case_types)]
                    struct AddVocabularySvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::AddVocabularyRequest>
                        for AddVocabularySvc<T>
                    {
                        type Response = super::VocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddVocabularyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_vocabulary(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddVocabularySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.VocabularyService/QueryVocabulary" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVocabularySvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::QueryVocabularyRequest>
                        for QueryVocabularySvc<T>
                    {
                        type Response = super::QueryVocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVocabularyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_vocabulary(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVocabularySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orion.VocabularyService/QueryVocabularyRandom" => {
                    #[allow(non_camel_case_types)]
                    struct QueryVocabularyRandomSvc<T: VocabularyService>(pub Arc<T>);
                    impl<T: VocabularyService>
                        tonic::server::UnaryService<super::QueryVocabularyRandomRequest>
                        for QueryVocabularyRandomSvc<T>
                    {
                        type Response = super::QueryVocabularyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryVocabularyRandomRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_vocabulary_random(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryVocabularyRandomSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: VocabularyService> Clone for VocabularyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VocabularyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VocabularyService> tonic::server::NamedService for VocabularyServiceServer<T> {
        const NAME: &'static str = "orion.VocabularyService";
    }
}
/// Generated server implementations.
pub mod story_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StoryServiceServer.
    #[async_trait]
    pub trait StoryService: Send + Sync + 'static {
        /// add a new story
        async fn add_story(
            &self,
            request: tonic::Request<super::AddStoryRequest>,
        ) -> Result<tonic::Response<super::StoryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StoryServiceServer<T: StoryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StoryService> StoryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StoryServiceServer<T>
    where
        T: StoryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orion.StoryService/AddStory" => {
                    #[allow(non_camel_case_types)]
                    struct AddStorySvc<T: StoryService>(pub Arc<T>);
                    impl<T: StoryService> tonic::server::UnaryService<super::AddStoryRequest> for AddStorySvc<T> {
                        type Response = super::StoryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddStoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_story(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddStorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: StoryService> Clone for StoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StoryService> tonic::server::NamedService for StoryServiceServer<T> {
        const NAME: &'static str = "orion.StoryService";
    }
}
