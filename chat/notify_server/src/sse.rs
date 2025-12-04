use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::{Sse, sse::Event},
};
use futures_util::stream::{self, Stream};
use tokio_stream::StreamExt;

use crate::AppState;

pub async fn sse_handler(
    State(_): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
