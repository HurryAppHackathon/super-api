use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    middlewares,
    structures::{AppState, User},
};

#[derive(Default)]
pub struct UserRequest {
    pub user: User,
}

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
#[async_trait]
impl FromRequest<AppState> for UserRequest {
    type Rejection = Response;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let token = req
            .headers()
            .get("Authorization")
            .ok_or(
                (
                    StatusCode::UNAUTHORIZED,
                    StatusCode::UNAUTHORIZED.to_string(),
                )
                    .into_response(),
            )?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    StatusCode::UNAUTHORIZED.to_string(),
                )
                    .into_response()
            })?;

        let user = middlewares::verify_user(token, state)?;
        Ok(Self { user })
    }
}
