use axum::{async_trait, extract::{FromRequest, Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};

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


pub async fn auth(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
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

    let user = verify_user(token, &state)?;
    Ok(next.run(req).await)
}