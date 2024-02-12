use crate::extractors::UserRequest;

use super::{AppState, Private, Session, User};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::*, Json, Router};
use serde::Deserialize;
#[derive(Deserialize)]
struct Register {
    username: String,
    password: String,
}

#[axum::debug_handler]
async fn register(
    // TODO: Create a validator extractor to get credentials
    State(state): State<AppState>,
    Json(payload): Json<Register>,
) -> impl IntoResponse {
    let user = User::new(&payload.username, &payload.password);
    let session = Session::new(user.id);
    let token = session.gen_token().unwrap(); // WARN: please do not crash
    state.sessions.lock().unwrap().push(session.clone());
    state.users.lock().unwrap().push(user);

    Json(token)
}
#[axum::debug_handler]
async fn all(State(state): State<AppState>) -> impl IntoResponse {
    return Json(state.users.lock().unwrap().clone());
}

#[axum::debug_handler]
async fn login(State(state): State<AppState>, Json(payload): Json<Register>) -> impl IntoResponse {
    let hash_password = payload.password;
    if let Some(user) = state.users.lock().unwrap().iter().find(|user| {
        if let Private::Hidden(pass) = &user.hash_password {
            **pass == *hash_password
        } else {
            false
        }
    }) {
        let session = Session::new(user.id);
        state.sessions.lock().unwrap().push(session.clone());
        (StatusCode::OK, Json(session.gen_token().unwrap())).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Json("Invalid credentials")).into_response()
    }
}

// TODO: move this into "/user" router
#[axum::debug_handler]
async fn me(State(_state): State<AppState>, UserRequest { user }: UserRequest) -> impl IntoResponse {
    Json(user)
}
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(me))
        .route("/register", post(register))
        .route("/login", post(login))
}
