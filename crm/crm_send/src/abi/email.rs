use std::vec;

use tonic::{Status, async_trait};

use crate::{
    Notification,
    abi::{self, To},
    pb::notification::{EmailMessage, SendRequest, SendResponse, send_request::Msg},
};

#[async_trait]
impl To for EmailMessage {
    async fn send(self, svc: &Notification) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::Email(self)).await.map_err(|e| {
            tracing::error!("send email message failed: {:?}", e);
            Status::internal("send email message failed")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(abi::to_ts()),
        })
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(msg: EmailMessage) -> Self {
        Self {
            msg: Some(Msg::Email(msg)),
        }
    }
}

impl From<EmailMessage> for Msg {
    fn from(msg: EmailMessage) -> Self {
        Self::Email(msg)
    }
}

impl EmailMessage {
    pub fn fake() -> Self {
        use fake::{Fake, faker::internet::en::SafeEmail};
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            from: SafeEmail().fake(),
            to: vec![SafeEmail().fake()],
            subject: "Test Subject".to_string(),
            body: "Test Body".to_string(),
        }
    }
}
