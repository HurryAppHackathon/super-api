//! Hurry app hackathon challenge
//! The primary focus of this application revolves around synchronizing view streams among party users.
//! Implementation of an authentication system will be omitted, as it falls outside the scope of this project's objectives.

//! Why using Arc<T> not the smart pointer of T, becuase some types i dont need the extra capacity to mutaute the thing.
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    serve, Json, Router,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
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

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
struct Message {
    id: Snowflake,
    content: Arc<str>,
    author: User,
}
impl Message {
    fn new(content: Arc<str>, author: User) -> Self {
        Self {
            id: Snowflake::generate(),
            content,
            author,
        }
    }
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
    messages: Vec<Arc<Message>>,
    video: Option<Video>,
}
impl Party {
    fn new(username: &str, name: &str) -> Self {
        let owner = User::new(username);
        Self {
            name: name.into(),
            owner,
            ..<_>::default()
        }
    }
}
#[derive(Clone)]
struct AppState {
    parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
    socket: Arc<Mutex<SocketIo>>,
}
#[derive(Deserialize, Debug, Serialize)]
struct M {
    author: Arc<str>,
    content: Arc<str>,
}

fn on_connect(socket: SocketRef) {
    let v = socket.req_parts();
    let auth = v.headers.get("Authorization");
    println!("{auth:?}");
    println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on("message", |socket: SocketRef, Data::<M>(data), Bin(bin)| {
        let message = Message::new(data.content, User::new(&data.author));
        println!("Received event: {:?} {:?}", message, bin);
        socket.bin(bin).emit("message-back", message).ok();
    });

    socket.on(
        "message-with-ack",
        |Data::<Value>(data), ack: AckSender, Bin(bin)| {
            println!("Received event: {:?} {:?}", data, bin);
            ack.bin(bin).send(data).ok();
        },
    );
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

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    io.ns("/custom", on_connect);

    let state = AppState {
        parties: Arc::new(Mutex::new(hash)),
        socket: Arc::new(Mutex::new(io)),
    };

    let party = Router::new()
        .route("/all", get(get_parties))
        .route("/", post(create_party))
        .route("/", delete(delete_party))
        // video routes
        .route("/:id/video", delete(remove_video))
        .route("/:id/video", post(attach_video))
        .route("/:id/messages", post(send_message))
        .route("/:id/messages", get(messages));

    let app = Router::new()
        .route("/", get(get_root))
        .nest("/party", party)
        // .route("/ws", get(realtime))
        .layer(layer)
        // .layer(CorsLayer::permissive())
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
struct PartyIdPath {
    id: Snowflake,
}
#[derive(Deserialize)]
struct CreateMessage {
    author: Arc<str>,
    content: Arc<str>,
}

async fn send_message(
    State(state): State<AppState>,
    Path(path): Path<PartyIdPath>,
    Json(payload): Json<CreateMessage>,
) -> impl IntoResponse {
    let guard = state.parties.lock().unwrap();

    if let Some(party) = guard.get(&path.id).cloned() {
        drop(guard);

        let mut guard = state.parties.lock().unwrap();
        let message = Arc::new(Message::new(payload.content, User::new(&*payload.author)));
        let mut party = Party {
            ..Party::clone(&party)
        };
        party.messages.push(Arc::clone(&message));
        guard.insert(path.id, Arc::new(party));

        state
            .socket
            .lock()
            .unwrap()
            .emit("message", Arc::clone(&message))
            .ok();

        return (StatusCode::OK, Json(message).into_response());
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        );
    };
}

async fn messages(
    State(state): State<AppState>,
    Path(path): Path<PartyIdPath>,
) -> impl IntoResponse {
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
        return (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        );
    };
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
    state
        .socket
        .lock()
        .unwrap()
        .emit("party-create", party.clone())
        .ok();
    Json(party)
}
async fn get_parties(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.parties.lock().unwrap();
    let hash: HashMap<_, _> = guard.clone(); // WARN: Deep clone
    Json(hash)
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
        (StatusCode::OK, Json("Party has been deleted"))
    } else {
        (StatusCode::NOT_FOUND, Json("Party not found"))
    }
}
#[derive(Deserialize)]
struct AttachVideo {
    video_url: Arc<str>,
}
#[axum::debug_handler]
async fn attach_video(
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
async fn remove_video(
    State(state): State<AppState>,
    Path(path): Path<PartyIdPath>
) -> impl IntoResponse {
    let read_guard = state.parties.lock().unwrap();
    if let Some(party) = read_guard.get(&path.id).cloned() {
        drop(read_guard);
        let mut write_guard = state.parties.lock().unwrap();
        let party = Arc::new(Party {
            video: None,
            ..Party::clone(&party) // WARN: Deep clone
        });
        write_guard.insert(
            path.id,
            Arc::clone(&party),
        );
        (StatusCode::OK, Json(party).into_response())
    } else {
        (
            StatusCode::NOT_FOUND,
            Json("Party not found").into_response(),
        )
    }
}
