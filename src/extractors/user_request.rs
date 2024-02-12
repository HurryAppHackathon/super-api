use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

use crate::structures::{AppState, User};

#[derive(Default)]
pub struct UserRequest {
    pub user: User,
}

#[async_trait]
impl FromRequestParts<AppState> for UserRequest {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(user) = parts.extensions.get::<User>().cloned() {
            Ok(Self { user })
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                StatusCode::UNAUTHORIZED.to_string(),
            )
                .into_response())
        }
    }
}
