use axum::{Router, middleware::from_fn};
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit,
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::models::user::User;

pub mod auth;
pub mod request_id;
pub mod server_time;

const HEADER_REQUEST_ID: &str = "x-request-id";
const HEADER_SERVER_TIME: &str = "x-server-time";

pub trait TokenVerify {
    type Error: std::fmt::Debug;

    fn verify(&self, token: &str) -> Result<User, Self::Error>;
}

pub fn set_layer(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(tracing::Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
            .layer(from_fn(request_id::set_request_id))
            .layer(server_time::ServerTimeLayer),
    )
}
