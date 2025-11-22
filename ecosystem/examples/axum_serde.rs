use std::sync::Arc;

use axum::{Json, Router, extract::State, routing};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    Layer as _,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

#[derive(Clone, Debug, Serialize)]
struct User {
    name: String,
    age: u8,
    city: String,
}

#[derive(Clone, Debug, Deserialize)]
struct UpdateUser {
    age: Option<u8>,
    city: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console_layer).init();

    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        city: "New York".to_string(),
    };
    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(&addr).await?;
    let app = Router::new()
        .route("/user", routing::get(user_handler))
        .route("/update-user", routing::put(update_user_handler))
        .with_state(user);

    info!("Starting server on {}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[tracing::instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    let user = user.lock().await;
    Json(user.clone())
}

#[tracing::instrument]
async fn update_user_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(update_user): Json<UpdateUser>,
) -> Json<User> {
    let mut user = user.lock().await;
    if let Some(age) = update_user.age {
        user.age = age;
    }
    if let Some(city) = update_user.city {
        user.city = city;
    }
    Json(user.clone())
}
