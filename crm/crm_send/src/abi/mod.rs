use chrono::Utc;
use futures::Stream;
use prost_types::Timestamp;
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::{Response, Status, async_trait};

use crate::{
    Notification, ResponseStream, ServiceResult,
    pb::notification::{SendRequest, SendResponse, send_request::Msg},
};

pub mod email;
pub mod in_app;
pub mod sms;

const CHANNEL_SIZE: usize = 256;

#[async_trait]
pub trait To {
    async fn send(self, svc: &Notification) -> Result<SendResponse, Status>;
}

impl Notification {
    pub async fn send<T>(&self, mut stream: T) -> ServiceResult<ResponseStream>
    where
        T: Stream<Item = Result<SendRequest, Status>> + Send + 'static + Unpin,
    {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        let svc = self.clone();

        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                let svc = svc.clone();
                match item {
                    Ok(req) => {
                        let res = match req.msg {
                            Some(Msg::Email(email)) => email.send(&svc).await,
                            Some(Msg::Sms(sms)) => sms.send(&svc).await,
                            Some(Msg::InApp(in_app)) => in_app.send(&svc).await,
                            None => {
                                tracing::error!("send request without message type");
                                continue;
                            }
                        };
                        if let Err(e) = tx.send(res).await {
                            tracing::error!("send response to channel failed: {:?}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        let stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(stream)))
    }
}

pub fn to_ts() -> Timestamp {
    let now = Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::AppConfig,
        pb::notification::{EmailMessage, InAppMessage, SmsMessage},
    };

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_send_should_work() -> Result<()> {
        let config = AppConfig::load()?;
        let svc = Notification::new(config).await;

        let stream = tokio_stream::iter(vec![
            Ok(EmailMessage::fake().into()),
            Ok(SmsMessage::fake().into()),
            Ok(InAppMessage::fake().into()),
        ]);

        let response = svc.send(stream).await?;
        let ret = response.into_inner().collect::<Vec<_>>().await;

        for item in ret {
            println!("item: {:?}", item?);
        }

        Ok(())
    }
}
