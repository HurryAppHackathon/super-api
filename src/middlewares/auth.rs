use axum::{
    extract::{Request, State},
    http::{request::Parts, StatusCode},
    middleware::{self, FromFnLayer, Next},
    response::{IntoResponse, Response},
    RequestExt,
};
use futures::TryFutureExt;

use crate::structures::{AppState, Session, User};

pub fn verify_user(token: &str, state: &AppState) -> Result<User, Response> {
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

pub async fn auth_parts(parts: &Parts, state: AppState) -> Result<User, Response> {
    let token = parts
        .headers
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

    let user = verify_user(token, &state)?;
    Ok(user)
}
pub async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let parts: Parts = req.extract_parts_with_state(&state).await.unwrap(); // never return an error
    let user = auth_parts(&parts, state).await?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
