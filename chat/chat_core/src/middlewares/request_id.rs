use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};

use crate::middlewares::HEADER_REQUEST_ID;

pub async fn set_request_id(mut req: Request, next: Next) -> Response {
    let id = match req.headers().get(HEADER_REQUEST_ID) {
        Some(id) => Some(id.clone()),
        None => {
            let id = uuid::Uuid::now_v7().to_string();
            match HeaderValue::from_str(&id) {
                Ok(value) => {
                    req.headers_mut().insert(HEADER_REQUEST_ID, value.clone());
                    Some(value)
                }
                Err(e) => {
                    tracing::error!("Failed to parse request id: {:?}", e);
                    None
                }
            }
        }
    };

    let mut response = next.run(req).await;
    let Some(id) = id else {
        return response;
    };
    response.headers_mut().insert(HEADER_REQUEST_ID, id);

    response
}
