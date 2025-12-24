use tonic::{Status, async_trait};

use crate::{
    Notification,
    abi::{self, To},
    pb::notification::{SendRequest, SendResponse, SmsMessage, send_request::Msg},
};

#[async_trait]
impl To for SmsMessage {
    async fn send(self, svc: &Notification) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::Sms(self)).await.map_err(|e| {
            tracing::error!("send sms message failed: {:?}", e);
            Status::internal("send sms message failed")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(abi::to_ts()),
        })
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(msg: SmsMessage) -> Self {
        Self {
            msg: Some(Msg::Sms(msg)),
        }
    }
}

impl From<SmsMessage> for Msg {
    fn from(msg: SmsMessage) -> Self {
        Self::Sms(msg)
    }
}

impl SmsMessage {
    pub fn fake() -> Self {
        use fake::Fake as _;
        use fake::faker::lorem::en::Sentence;
        use fake::faker::phone_number::en::PhoneNumber;
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            from: PhoneNumber().fake(),
            to: vec![PhoneNumber().fake()],
            body: Sentence(10..15).fake(),
        }
    }
}
