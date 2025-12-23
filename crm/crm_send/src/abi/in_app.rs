use fake::Fake as _;
use tonic::{Status, async_trait};

use crate::{
    Notification,
    abi::{self, To},
    pb::notification::{InAppMessage, SendRequest, SendResponse, send_request::Msg},
};

#[async_trait]
impl To for InAppMessage {
    async fn send(self, svc: &Notification) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();

        svc.sender.send(Msg::InApp(self)).await.map_err(|e| {
            tracing::error!("send in app message failed: {:?}", e);
            Status::internal("send in app message failed")
        })?;

        Ok(SendResponse {
            message_id,
            timestamp: Some(abi::to_ts()),
        })
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(msg: InAppMessage) -> Self {
        Self {
            msg: Some(Msg::InApp(msg)),
        }
    }
}

impl InAppMessage {
    pub fn fake() -> Self {
        use fake::faker::lorem::en::Sentence;
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            title: Sentence(1..3).fake(),
            body: Sentence(10..15).fake(),
        }
    }
}
