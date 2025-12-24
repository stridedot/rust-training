use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::{
    config::AppConfig,
    pb::metadata::{
        Content, MaterializeRequest,
        metadata_service_server::{MetadataService, MetadataServiceServer},
    },
};

pub mod abi;
pub mod config;
pub mod pb;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

pub struct CrmMetadata {
    #[allow(dead_code)]
    config: AppConfig,
}

#[async_trait]
impl MetadataService for CrmMetadata {
    #[doc = " Server streaming response type for the Materialize method."]
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let stream = request.into_inner();

        self.materialize(stream).await
    }
}

impl CrmMetadata {
    pub async fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn into_server(self) -> MetadataServiceServer<Self> {
        MetadataServiceServer::new(self)
    }
}
