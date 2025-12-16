use anyhow::Result;

use tonic::{Request, Response, Status, async_trait};

use crate::{
    config::AppConfig,
    pb::crm::{
        RecallRequest, RecallResponse, RemindRequest, RemindResponse, WelcomeRequest,
        WelcomeResponse,
        crm_service_server::{CrmService, CrmServiceServer},
    },
};

pub mod config;
pub mod pb;

pub struct CrmGateway {
    #[allow(dead_code)]
    config: AppConfig,
}

#[async_trait]
impl CrmService for CrmGateway {
    async fn welcome(
        &self,
        request: Request<WelcomeRequest>,
    ) -> Result<Response<WelcomeResponse>, Status> {
        let resp = WelcomeResponse {
            id: request.into_inner().id,
        };
        Ok(Response::new(resp))
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
        Ok(Self { config })
    }

    pub fn into_server(self) -> CrmServiceServer<Self> {
        CrmServiceServer::new(self)
    }
}
