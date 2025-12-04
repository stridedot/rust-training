use axum::{
    extract::{FromRequestParts, Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse as _, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use serde::Deserialize;

use crate::{middlewares::TokenVerify, utils::ApiResponse};

#[derive(Debug, Deserialize)]
struct Params {
    token: String,
}

pub async fn verify_token<T>(State(state): State<T>, req: Request, next: Next) -> Response
where
    T: TokenVerify + Clone + Send + Sync + 'static,
{
    let (mut parts, body) = req.into_parts();
    println!("1111,{:?}", parts);

    let token =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer.token().to_string(),
            Err(e) => {
                if e.is_missing() {
                    match Query::<Params>::from_request_parts(&mut parts, &state).await {
                        Ok(Query(params)) => params.token,
                        Err(e) => {
                            let msg = "failed to parse token from query";
                            tracing::error!("{}, error: {}", msg, e);
                            return ApiResponse::<()>::error(StatusCode::UNAUTHORIZED, msg)
                                .into_response();
                        }
                    }
                } else {
                    let msg = format!("Failed to parse token from header, error: {:?}", e);
                    tracing::error!("{}", msg);
                    return ApiResponse::<()>::error(StatusCode::UNAUTHORIZED, msg).into_response();
                }
            }
        };

    let req = match state.verify(&token) {
        Ok(user) => {
            let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(user);
            req
        }
        Err(e) => {
            let msg = format!("Failed to verify token, error: {:?}", e);
            tracing::error!("{}", msg);
            return ApiResponse::<()>::error(StatusCode::FORBIDDEN, msg).into_response();
        }
    };

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::models::user::User;
    use crate::utils::jwt::{DecodingKey, EncodingKey};

    use axum::body::Body;
    use axum::middleware::from_fn_with_state;
    use axum::routing::get;
    use axum::{Router, body};
    use jwt_simple::reexports::serde_json;
    use std::sync::Arc;
    use tower::ServiceExt;

    #[derive(Clone)]
    struct AppState(Arc<AppStateInner>);

    struct AppStateInner {
        sign_key: EncodingKey,
        verify_key: DecodingKey,
    }

    impl TokenVerify for AppState {
        type Error = ();

        fn verify(&self, token: &str) -> Result<User, Self::Error> {
            self.0.verify_key.verify(token).map_err(|_| ())
        }
    }

    #[tokio::test]
    async fn test_verify_token() -> anyhow::Result<()> {
        let enc_pem = include_str!("../../fixtures/enc.pem");
        let dec_pem = include_str!("../../fixtures/dec.pem");

        let enc_key = EncodingKey::load(enc_pem)?;
        let dec_key = DecodingKey::load(dec_pem)?;

        let state = AppState(Arc::new(AppStateInner {
            sign_key: enc_key,
            verify_key: dec_key,
        }));

        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
            .with_state(state.clone());

        let user = User {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "123456".to_string(),
            workspace_id: 1,
            created_at: chrono::Utc::now(),
        };
        let token = state.0.sign_key.sign(user)?;

        let req = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;

        let body = body::to_bytes(resp.into_body(), usize::MAX).await?;
        assert_eq!(std::str::from_utf8(&body)?, "Hello, World!");

        let req = Request::builder()
            .uri(format!("/?token={}", token))
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;

        let body = body::to_bytes(resp.into_body(), usize::MAX).await?;
        assert_eq!(std::str::from_utf8(&body)?, "Hello, World!");

        let req = Request::builder().uri("/").body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;

        let body = body::to_bytes(resp.into_body(), usize::MAX).await?;
        let api_resp: ApiResponse<()> = serde_json::from_slice(&body)?;
        assert_eq!(api_resp.code, 401);

        let req = Request::builder()
            .uri("/")
            .header("Authorization", "Bearer bad token")
            .body(Body::empty())?;
        let resp = app.clone().oneshot(req).await?;

        let body = body::to_bytes(resp.into_body(), usize::MAX).await?;
        let api_resp: ApiResponse<()> = serde_json::from_slice(&body)?;
        assert_eq!(api_resp.code, 403);

        Ok(())
    }
}
