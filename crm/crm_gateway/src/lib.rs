use anyhow::Result;

use crm_metadata::pb::metadata::metadata_service_client::MetadataServiceClient;
use crm_send::pb::notification::notification_service_client::NotificationServiceClient;
use crm_stat::pb::user_stat::user_stat_service_client::UserStatServiceClient;
use tonic::{Request, Response, Status, async_trait, transport::Channel};

use crate::{
    config::AppConfig,
    pb::crm::{
        RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
        WelcomeResponse,
        crm_service_server::{CrmService, CrmServiceServer},
    },
};

pub mod abi;
pub mod config;
pub mod pb;

pub struct CrmGateway {
    #[allow(dead_code)]
    config: AppConfig,
    user_stat: UserStatServiceClient<Channel>,
    notification: NotificationServiceClient<Channel>,
    metadata: MetadataServiceClient<Channel>,
}

#[async_trait]
impl CrmService for CrmGateway {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        self.welcome(request.into_inner()).await
    }

    async fn recall(
        &self,
        request: Request<RecallRequest>,
    ) -> Result<Response<RecallResponse>, Status> {
        let resp = RecallResponse {
            id: request.into_inner().id,
        };
        Ok(Response::new(resp))
    }

    async fn remind(
        &self,
        request: Request<RemindRequest>,
    ) -> Result<Response<RemindResponse>, Status> {
        let resp = RemindResponse {
            id: request.into_inner().id,
        };
        Ok(Response::new(resp))
    }
}

impl CrmGateway {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let user_stat = UserStatServiceClient::connect(config.server.user_stat_url.clone()).await?;
        let notification =
            NotificationServiceClient::connect(config.server.notify_url.clone()).await?;
        let metadata = MetadataServiceClient::connect(config.server.metadata_url.clone()).await?;

        Ok(Self {
            config,
            user_stat,
            notification,
            metadata,
        })
    }

    pub fn into_server(self) -> CrmServiceServer<Self> {
        CrmServiceServer::new(self)
    }
}
