use std::{collections::HashSet, sync::Arc};

use chat_core::models::{chat::Chat, message::Message};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgListener;

use crate::AppState;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AppEvent {
    NewChat(Chat),
    AddToChat(Chat),
    RemoveFromChat(Chat),
    NewMessage(Message),
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatUpdated {
    op: String,
    old: Option<Chat>,
    new: Option<Chat>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ChatMessageCreated {
    message: Message,
    user_ids: Vec<i64>,
}

pub async fn setup_pg_listener(state: AppState) -> anyhow::Result<()> {
    let mut listener = PgListener::connect(&state.config.server.pg_url).await?;
    listener.listen("chat_updated").await?;
    listener.listen("chat_message_created").await?;

    let mut stream = listener.into_stream();

    tokio::spawn(async move {
        while let Some(Ok(event)) = stream.next().await {
            let notification = Notification::load(event.channel(), event.payload()).await?;

            for user_id in notification.user_ids {
                let Some(tx) = &state.users.get(&user_id) else {
                    continue;
                };
                if let Err(e) = tx.send(notification.event.clone()) {
                    tracing::error!("Failed to send event to user {}: {:?}", user_id, e);
                }
            }
        }

        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

struct Notification {
    user_ids: HashSet<i64>,
    event: Arc<AppEvent>,
}

impl Notification {
    async fn load(channel: &str, payload: &str) -> anyhow::Result<Self> {
        match channel {
            "chat_updated" => {
                let payload: ChatUpdated = serde_json::from_str(payload)?;
                let user_ids =
                    get_effected_chat_user_ids(payload.old.as_ref(), payload.new.as_ref());

                let event = match payload.op.to_lowercase().as_str() {
                    "insert" => AppEvent::NewChat(payload.new.unwrap()),
                    "update" => AppEvent::AddToChat(payload.new.unwrap()),
                    "delete" => AppEvent::RemoveFromChat(payload.old.unwrap()),
                    _ => return Err(anyhow::anyhow!("Invalid operation: {}", payload.op)),
                };

                Ok(Self {
                    user_ids,
                    event: Arc::new(event),
                })
            }
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(payload)?;
                let event = AppEvent::NewMessage(payload.message);
                Ok(Self {
                    user_ids: payload.user_ids.into_iter().collect(),
                    event: Arc::new(event),
                })
            }
            _ => Err(anyhow::anyhow!("Invalid notification type: {}", channel)),
        }
    }
}

fn get_effected_chat_user_ids(old: Option<&Chat>, new: Option<&Chat>) -> HashSet<i64> {
    match (old, new) {
        (None, Some(new_chat)) => new_chat.user_ids.iter().copied().collect(),
        (Some(old_chat), None) => old_chat.user_ids.iter().copied().collect(),
        (Some(old_chat), Some(new_chat)) => {
            if old_chat.user_ids == new_chat.user_ids {
                HashSet::new()
            } else {
                let mut user_ids = old_chat.user_ids.iter().copied().collect::<HashSet<_>>();
                user_ids.extend(new_chat.user_ids.iter().copied());
                user_ids
            }
        }
        _ => HashSet::new(),
    }
}
