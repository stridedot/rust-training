use std::{convert::Infallible, time::Duration};

use axum::{
    Extension,
    extract::State,
    response::{Sse, sse::Event},
};
use chat_core::models::user::User;
use futures_util::stream::Stream;
use tokio::sync::broadcast;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::{AppState, notify::AppEvent};

const CHANNEL_CAPACITY: usize = 256;

pub async fn sse_handler(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let user_id = user.id;
    let users = &state.users;

    let rx = if let Some(tx) = users.get(&user_id) {
        tx.subscribe()
    } else {
        let (tx, rx) = broadcast::channel(CHANNEL_CAPACITY);
        state.users.insert(user_id, tx);
        rx
    };
    tracing::info!("User {} subscribed to SSE", user_id);

    let stream = BroadcastStream::new(rx).filter_map(|v| v.ok()).map(|v| {
        let name = match v.as_ref() {
            AppEvent::NewChat(_) => "NewChat",
            AppEvent::AddToChat(_) => "AddToChat",
            AppEvent::RemoveFromChat(_) => "RemoveFromChat",
            AppEvent::NewMessage(_) => "NewMessage",
        };

        match serde_json::to_string(&v) {
            Ok(v) => Ok(Event::default().event(name).data(v)),
            Err(e) => {
                tracing::error!("Failed to serialize event: {:?}", e);
                Ok(Event::default().comment("serialize failed"))
            }
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
