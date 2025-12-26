use chrono::{Duration, Utc};
use crm_metadata::pb::metadata::{Content, MaterializeRequest};
use crm_send::pb::notification::{EmailMessage, SendRequest, send_request::Msg};
use crm_stat::pb::user_stat::QueryRequest;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

pub mod auth;

use crate::{
    CrmGateway,
    pb::crm::{WelcomeRequest, WelcomeResponse},
};

impl CrmGateway {
    pub async fn welcome(&self, req: WelcomeRequest) -> Result<Response<WelcomeResponse>, Status> {
        let WelcomeRequest {
            id: request_id,
            interval,
            content_ids,
        } = req;
        let d1 = Utc::now() - Duration::days(interval as _);
        let d2 = d1 + Duration::days(1);
        let query = QueryRequest::new_with_dt("created_at", d1, d2);

        let mut res_user_stats = self.user_stat.clone().query(query).await?.into_inner();

        let query = MaterializeRequest::new_with_ids(content_ids);
        let contents = self.metadata.clone().materialize(query).await?.into_inner();

        let contents: Vec<Content> = contents
            .filter_map(|v| async move { v.ok() })
            .collect()
            .await;

        let contents = Arc::new(contents);

        let (tx, rx) = mpsc::channel(1024);

        let message_id = request_id.clone();
        let sender = self.config.server.from.clone();
        tokio::spawn(async move {
            while let Some(Ok(user)) = res_user_stats.next().await {
                let contents = contents.clone();

                let email = Msg::Email(EmailMessage {
                    message_id: message_id.clone(),
                    from: sender.clone(),
                    to: vec![user.email],
                    subject: "Welcome".to_string(),
                    body: format!("{:?}", contents),
                });
                let req = SendRequest { msg: Some(email) };

                if let Err(e) = tx.send(req).await {
                    tracing::error!("send email failed: {:?}", e);
                };
            }
        });

        let reqs = ReceiverStream::new(rx);
        self.notification.clone().send(reqs).await?;

        Ok(Response::new(WelcomeResponse { id: request_id }))
    }
}
