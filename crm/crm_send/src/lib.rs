use std::{ops::Deref, pin::Pin, sync::Arc, time::Duration};

use futures::Stream;
use tokio::{sync::mpsc, time::sleep};
use tonic::{Request, Response, Status, Streaming, async_trait};

use crate::{
    config::AppConfig,
    pb::notification::{
        SendRequest, SendResponse,
        notification_service_server::{NotificationService, NotificationServiceServer},
        send_request::Msg,
    },
};

pub mod abi;
pub mod config;
pub mod pb;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<SendResponse, Status>> + Send>>;

#[derive(Clone)]
pub struct Notification {
    inner: Arc<NotificationInner>,
}

pub struct NotificationInner {
    #[allow(dead_code)]
    config: AppConfig,
    sender: mpsc::Sender<Msg>,
}

#[async_trait]
impl NotificationService for Notification {
    #[doc = " Server streaming response type for the Send method."]
    type SendStream = ResponseStream;

    async fn send(
        &self,
        request: Request<Streaming<SendRequest>>,
    ) -> ServiceResult<Self::SendStream> {
        let stream = request.into_inner();
        self.send(stream).await
    }
}

impl Notification {
    pub async fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(NotificationInner {
                config,
                sender: dummy_send(),
            }),
        }
    }

    pub fn into_server(self) -> NotificationServiceServer<Self> {
        NotificationServiceServer::new(self)
    }
}

impl Deref for Notification {
    type Target = NotificationInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn dummy_send() -> mpsc::Sender<Msg> {
    let (tx, mut rx) = mpsc::channel(256);

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            tracing::info!("dummy send: {:?}", msg);
            sleep(Duration::from_secs(1)).await;
        }
    });

    tx
}
