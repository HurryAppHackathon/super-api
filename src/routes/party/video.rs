use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::*,
    Json, Router,
};
use serde::Deserialize;

use super::{AppState, Party, Snowflake, Video};

#[derive(Deserialize)]
struct PartyIdPath {
    id: Snowflake,
}

#[derive(Deserialize)]
struct AttachVideo {
    video_url: Arc<str>,
}
#[axum::debug_handler]
async fn attach(
    State(state): State<AppState>,
    Path(path): Path<PartyIdPath>,
    Json(payload): Json<AttachVideo>,
) -> impl IntoResponse {
    let read_guard = state.parties.lock().unwrap();
    if let Some(party) = read_guard.get(&path.id).cloned() {
        drop(read_guard);
        let mut write_guard = state.parties.lock().unwrap();
        let party = Arc::new(Party {
            video: Some(Video::new(&payload.video_url)),
            ..Party::clone(&party) // WARN: Deep clone
        });
        write_guard.insert(path.id, Arc::clone(&party));
        (StatusCode::OK, Json(party).into_response())
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        )
    }
}

#[axum::debug_handler]
async fn remove(State(state): State<AppState>, Path(path): Path<PartyIdPath>) -> impl IntoResponse {
    let read_guard = state.parties.lock().unwrap();
    if let Some(party) = read_guard.get(&path.id).cloned() {
        drop(read_guard);
        let mut write_guard = state.parties.lock().unwrap();
        let party = Arc::new(Party {
            video: None,
            ..Party::clone(&party) // WARN: Deep clone
        });
        write_guard.insert(path.id, Arc::clone(&party));
        (StatusCode::OK, Json(party).into_response())
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        )
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(attach))
        .route("/", delete(remove))
}
