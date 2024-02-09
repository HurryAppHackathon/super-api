mod messages;
mod video;

use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::*, Json, Router};
use serde::Deserialize;

use crate::{extractors::UserRequest, structures::*};


#[derive(Deserialize)]
struct CreateParty {
    name: Arc<str>,
    username: Arc<str>,
}
#[derive(Deserialize)]
struct DeleteParty {
    id: Snowflake,
}

async fn all(
    State(state): State<AppState>,
    UserRequest { user }: UserRequest,
) -> impl IntoResponse {
    let guard = state.parties.lock().unwrap();
    let hash: HashMap<_, _> = guard.clone(); // WARN: Deep clone
    Json(user)
}

#[axum::debug_handler]
async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateParty>,
) -> impl IntoResponse {
    let party = Arc::from(Party::new(User::new("test", "test"), &payload.name)); // TODO: get user
    state
        .parties
        .lock()
        .unwrap()
        .insert(Snowflake::generate(), Arc::clone(&party));
    // state
    //     .socket
    //     .lock()
    //     .unwrap()
    //     .emit("party-create", party.clone())
    //     .ok(); // FIXME
    Json(party)
}

#[axum::debug_handler]
async fn remove(
    State(state): State<AppState>,
    Json(payload): Json<DeleteParty>,
) -> impl IntoResponse {
    if state.parties.lock().unwrap().remove(&payload.id).is_some() {
        (StatusCode::OK, Json("Party has been deleted"))
    } else {
        (StatusCode::NOT_FOUND, Json("Party not found"))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/:id/messages", messages::routes())
        .nest("/:id/video", video::routes())
        .route("/all", get(all))
        .route("/", post(create))
        .route("/", delete(remove))
}
