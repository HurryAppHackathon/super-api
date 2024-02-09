use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::*,
    Json, Router,
};
use serde::Deserialize;

use super::{AppState, Message, Party, Snowflake, User};

#[derive(Deserialize)]
struct PartyIdPath {
    id: Snowflake,
}
#[derive(Deserialize)]
struct CreateMessage {
    author: Arc<str>,
    content: Arc<str>,
}

async fn create(
    State(state): State<AppState>,
    Path(path): Path<PartyIdPath>,
    Json(payload): Json<CreateMessage>,
) -> impl IntoResponse {
    let guard = state.parties.lock().unwrap();

    if let Some(party) = guard.get(&path.id).cloned() {
        drop(guard);

        let mut guard = state.parties.lock().unwrap();
        let message = Arc::new(Message::new(
            payload.content,
            User::new(&payload.author, "as"),
        )); // TODO: get user
        let mut party = Party {
            ..Party::clone(&party)
        };
        party.messages.push(Arc::clone(&message));
        guard.insert(path.id, Arc::new(party));

        // state
        //     .socket
        //     .lock()
        //     .unwrap()
        //     .emit("message", Arc::clone(&message))
        //     .ok(); // FIXME

        (StatusCode::OK, Json(message).into_response())
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        )
    }
}

async fn all(State(state): State<AppState>, Path(path): Path<PartyIdPath>) -> impl IntoResponse {
    let hash = state.parties.lock().unwrap();

    if let Some(party) = hash.get(&path.id).cloned() {
        return (
            StatusCode::OK,
            Json(
                party
                    .messages
                    .iter()
                    .map(|c| c.as_ref())
                    .collect::<Vec<&Message>>(),
            )
            .into_response(),
        );
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        )
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(create)).route("/", get(all))
}
