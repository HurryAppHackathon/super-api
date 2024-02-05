//! Hurry app hackathon challenge
//! The primary focus of this application revolves around synchronizing view streams among party users.
//! Implementation of an authentication system will be omitted, as it falls outside the scope of this project's objectives.

//! Why using Arc<T> not the smart pointer of T, becuase some types i dont need the extra capacity to mutaute the thing.
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    serve, Json, Router,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

mod config;
mod error;
mod prelude;
mod snowflake;

use config::*;
use prelude::*;
use snowflake::Snowflake;

#[derive(Clone, Serialize)]
struct User {
    id: Snowflake, 
    username: Arc<str>,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: <_>::default(),
            username: Arc::from(""),
        }
    }
}
impl User {
    fn new(username: &str) -> Self {
        Self {
            username: username.into(),
            ..<_>::default()
        }
    }
}

#[derive(Clone, Serialize)]
struct Message {
    id: Snowflake, 
    content: Arc<str>,
    author: User,
}
impl Default for Message {
    fn default() -> Self {
        Self {
            content: Arc::from(""),
            id: <_>::default(),
            author: <_>::default(),
        }
    }
}

#[derive(Clone, Serialize)]
struct Video {
    video_url: Arc<str>,
}
impl Video {
    fn new(video_url: &str) -> Self {
        Self {
            video_url: video_url.into(),
        }
    }
}
impl Default for Video {
    fn default() -> Self {
        Self {
            video_url: Arc::from(""),
        }
    }
}
#[derive(Default, Clone, Serialize)]
struct Party {
    name: String,
    owner: User,
    messages: Vec<Message>,
    video: Option<Video>,
}
impl Party {
    fn new(username: &str, name: &str) -> Self {
        let owner = User::new(username);
        Self {
            name: name.into(),
            owner,
            ..Default::default()
        }
    }
}
#[derive(Default, Clone)]
struct AppState {
    parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // ! some performance tests
    #[allow(unused_mut)]
    let mut hash = HashMap::new();
    // let mut rng = rand::thread_rng();
    // let mut i = 0;
    // loop {
    //     hash.insert(rng.gen(), Arc::new(Party::new("username", "name")));
    //     if i == 20000 {
    //         break;
    //     }
    //     i += 1;
    // }
    let state = AppState {
        parties: Arc::new(Mutex::new(hash)),
    };

    let app = Router::new()
        .route("/", get(get_root))
        .route("/parties", get(get_parties))
        .route("/create_party", post(create_party))
        .route("/attach_video", post(attach_video))
        .route("/remove_video", delete(remove_video))
        .route("/delete_party", delete(delete_party))
        .with_state(state);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT)).await?;

    println!("🚀 Server is running: http://{}", listener.local_addr()?);

    serve(listener, app).await?;

    Ok(())
}

async fn get_root() -> impl IntoResponse {
    "Hello from axum!"
}

#[derive(Deserialize)]
struct CreateParty {
    name: Arc<str>,
    username: Arc<str>,
}

#[axum::debug_handler]
async fn create_party(
    State(state): State<AppState>,
    Json(payload): Json<CreateParty>,
) -> impl IntoResponse {
    let party = Arc::from(Party::new(&payload.username, &payload.name));
    state
        .parties
        .lock()
        .unwrap()
        .insert(Snowflake::generate(), Arc::clone(&party));
    return Json(party);
}
async fn get_parties(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.parties.lock().unwrap();
    let hash: HashMap<_, _> = guard.clone(); // WARN: Deep clone
    return Json(hash);
}
#[derive(Deserialize)]
struct DeleteParty {
    id: Snowflake, 
}
async fn delete_party(
    State(state): State<AppState>,
    Json(payload): Json<DeleteParty>,
) -> impl IntoResponse {
    if let Some(_) = state.parties.lock().unwrap().remove(&payload.id) {
        return (StatusCode::OK, Json("Party has been deleted"));
    } else {
        return (StatusCode::NOT_FOUND, Json("Party not found"));
    }
}
#[derive(Deserialize)]
struct AttachVideo {
    id: Snowflake,
    video_url: Arc<str>,
}
#[axum::debug_handler]
async fn attach_video(
    State(state): State<AppState>,
    Json(payload): Json<AttachVideo>,
) -> impl IntoResponse {
    let read_guard = state.parties.lock().unwrap();
    if let Some(party) = read_guard.get(&payload.id).cloned() {
        drop(read_guard);
        let mut write_guard = state.parties.lock().unwrap();
        let party = Arc::new(Party {
            video: Some(Video::new(&payload.video_url)),
            ..Party::clone(&party) // WARN: Deep clone
        });
        write_guard.insert(
            payload.id,
            Arc::clone(&party),
        );
        return (StatusCode::OK, Json(party).into_response());
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        );
    }
}

#[derive(Deserialize)]
struct RemoveVideo {
    id: Snowflake, 
}
#[axum::debug_handler]
async fn remove_video(
    State(state): State<AppState>,
    Json(payload): Json<RemoveVideo>,
) -> impl IntoResponse {
    let read_guard = state.parties.lock().unwrap();
    if let Some(party) = read_guard.get(&payload.id).cloned() {
        drop(read_guard);
        let mut write_guard = state.parties.lock().unwrap();
        write_guard.borrow_mut().insert(
            payload.id,
            Arc::new(Party {
                video: None,
                ..Party::clone(&party) // WARN: Deep clone
            }),
        );
        return (StatusCode::OK, Json(party).into_response());
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        );
    }
}