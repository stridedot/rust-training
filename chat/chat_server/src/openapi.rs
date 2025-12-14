use axum::Router;
use chat_core::models::{
    chat::Chat,
    message::Message,
    user::{ChatUser, User},
    workspace::Workspace,
};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    AppState,
    requests::{
        chat::{CreateChatReq, DeleteChatReq, UpdateChatReq},
        message::{MessageListRequest, MessageSendRequest},
        user::{SignInReq, SignUpReq},
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::sign_up,
        crate::handlers::auth::sign_in,
        crate::handlers::user::user_list,
        crate::handlers::chat::chat_create,
        crate::handlers::chat::chat_update,
        crate::handlers::chat::chat_delete,
        crate::handlers::chat::chat_detail,
        crate::handlers::chat::chat_list,
        crate::handlers::message::message_list,
        crate::handlers::message::message_send,

    ),
    components(
        schemas(
            SignUpReq,
            SignInReq,
            User,
            ChatUser,
            Workspace,

            CreateChatReq,
            UpdateChatReq,
            DeleteChatReq,
            Chat,

            MessageListRequest,
            MessageSendRequest,
            Message
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "chat", description = "Chat management API")
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

pub trait OpenApiRouter {
    fn openapi(self) -> Self;
}

impl OpenApiRouter for Router<AppState> {
    fn openapi(self) -> Self {
        self.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    }
}
