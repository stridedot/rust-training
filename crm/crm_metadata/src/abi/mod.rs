use std::collections::HashSet;

use chrono::{DateTime, Days, Utc};
use fake::{
    Fake, Faker,
    faker::{chrono::en::DateTimeBetween, lorem::zh_cn::Sentence, name::zh_cn},
};
use futures::{Stream, StreamExt as _, stream};
use prost_types::Timestamp;
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};

use crate::{
    CrmMetadata, ResponseStream, ServiceResult,
    pb::metadata::{Content, MaterializeRequest, Publisher},
};

const CHANNEL_SIZE: usize = 256;

impl CrmMetadata {
    pub async fn materialize<T>(&self, stream: T) -> ServiceResult<ResponseStream>
    where
        T: Stream<Item = Result<MaterializeRequest, Status>> + Send + 'static + Unpin,
    {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);

        tokio::spawn(async move {
            let mut stream = stream;

            while let Some(item) = stream.next().await {
                match item {
                    Ok(req) => {
                        let content = Content::materialize(req.id);
                        if let Err(e) = tx.send(Ok(content)).await {
                            tracing::error!("send content to channel failed: {:?}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        // 如果接收流出错，也可以把错误发给客户端
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

impl Content {
    pub fn materialize(id: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            name: zh_cn::Name().fake(),
            description: Sentence(5..15).fake(),
            publishers: (1..=9).map(|_| Publisher::new()).collect(),
            url: format!("https://www.youtube.com/watch?v={id}"),
            image: format!("https://placehold.co/800x450?text={id}"),
            r#type: Faker.fake(),
            views: rng.gen_range(123432..100000000),
            dislikes: rng.gen_range(0..10000),
            likes: rng.gen_range(0..10000),
            created_at: created_at(),
        }
    }
}

impl Publisher {
    pub fn new() -> Self {
        Self {
            id: (1000..99999).fake(),
            name: zh_cn::Name().fake(),
            avatar: "https://placehold.co/400x400".to_string(),
        }
    }
}

fn created_at() -> Option<Timestamp> {
    let now = Utc::now();

    let start = now.checked_sub_days(Days::new(365))?;
    let date: DateTime<Utc> = DateTimeBetween(start, now).fake();

    Some(Timestamp {
        seconds: date.timestamp(),
        nanos: date.timestamp_subsec_nanos() as _,
    })
}

impl MaterializeRequest {
    pub fn new_with_ids(ids: Vec<u32>) -> impl Stream<Item = Self> {
        let reqs: HashSet<_> = ids.into_iter().map(|id| Self { id }).collect();
        stream::iter(reqs)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    use super::*;

    #[tokio::test]
    async fn test_materialize_should_work() -> anyhow::Result<()> {
        let config = AppConfig::load()?;
        let service = CrmMetadata::new(config).await;

        let stream = tokio_stream::iter(vec![
            Ok(MaterializeRequest { id: 1 }),
            Ok(MaterializeRequest { id: 2 }),
            Ok(MaterializeRequest { id: 3 }),
        ]);

        let response = service.materialize(stream).await?;
        let mut stream = response.into_inner();

        while let Some(item) = stream.next().await {
            let item = item?;
            println!("{:?}", item);
        }

        Ok(())
    }
}
