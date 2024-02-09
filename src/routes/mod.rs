mod auth;
mod party;

use axum::{async_trait, extract::{FromRequest, Request, State}, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Response}, routing::*, Router};

use crate::structures::*;

async fn root() -> impl IntoResponse {
    "Hi"
}

pub fn mount(app: Router<AppState>, state: AppState) -> Router<AppState> {
    app.route("/", get(root))
        .nest("/auth", auth::routes())
        .nest("/party", party::routes().layer(middleware::from_fn_with_state(state , auth_middleware)))
}



fn verify_user(req: &Request, state: &AppState) -> Result<User, Response> {
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

    let local_session = Session::decode(token.to_string()).map_err(|e| e.into_response())?;
    let sessions = state.sessions.lock().unwrap();

    let session = sessions.iter().find(|s| s.id == local_session.id).ok_or(
        (
            StatusCode::UNAUTHORIZED,
            StatusCode::UNAUTHORIZED.to_string(),
        )
            .into_response(),
    )?;

    let users = state.users.lock().unwrap();

    let user = users
        .iter()
        .find(|u| u.id == session.user_id)
        .cloned()
        .ok_or(
            (
                StatusCode::UNAUTHORIZED,
                StatusCode::UNAUTHORIZED.to_string(),
            )
                .into_response(),
        )?;
    Ok(user)
}


async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let user = verify_user(&req, &state)?;
    Ok(next.run(req).await)
}
#[derive(Default)]
pub struct UserRequest {
    pub user: User,
}

// we must implement `FromRequest` (and not `FromRequestParts`) to consume the body
#[async_trait]
impl FromRequest<AppState> for UserRequest {
    type Rejection = Response;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let user = verify_user(&req, &state)?;
        Ok(Self { user })
    }
}