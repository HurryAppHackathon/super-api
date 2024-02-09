#![allow(unused)] // TODO: remove on production
//! Hurry app hackathon challenge
//! The primary focus of this application revolves around synchronizing view streams among party users.

//! Why using Arc<T> not the smart pointer of T, becuase some types i dont need the extra capacity to mutaute the thing.
#[macro_use]
extern crate serde_with;

#[macro_use]
extern crate lazy_static;

use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    serve, Router,
};

use serde::{Deserialize, Serialize};

use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use structures::{Session, User};
use tokio::net::TcpListener;

mod config;
mod error;
mod prelude;

mod routes;
mod structures;

use config::*;
use prelude::*;

use crate::structures::AppState;

#[derive(Deserialize, Debug, Serialize)]
struct M {
    author: Arc<str>,
    content: Arc<str>,
}
#[derive(Deserialize, Debug)]
struct HandShake {
    token: String,
}

fn on_connect(_socket: SocketRef, Data(data): Data<HandShake>) {
    println!("{data:?}");
    // socket.disconnect().ok();
    //
    //     println!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    //     // socket.bin(binary)
    //     socket.on("join", |socket: SocketRef, Data::<Value>(data), Bin(bin)| {
    //         println!("joined?");
    //         socket.bin(bin).emit("joined", ()).ok();
    //
    //         // let message = Message::new(data.content, User::new(&data.author));
    //         // println!("Received event: {:?} {:?}", message, bin);
    //         // socket.bin(bin).emit("message-back", message).ok();
    //     });
    //
    //     // socket.on(
    //     //     "message-with-ack",
    //     //     |Data::<Value>(data), ack: AckSender, Bin(bin)| {
    //     //         println!("Received event: {:?} {:?}", data, bin);
    //     //         ack.bin(bin).send(data).ok();
    //     //     },
    //     // );
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", on_connect);
    let state = AppState {
        parties: Arc::new(Mutex::new(HashMap::new())),
        socket: Arc::new(Mutex::new(io)),
        users: Arc::new(Mutex::new(vec![])),
        sessions: Arc::new(Mutex::new(vec![])),
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT)).await?;

    println!("🚀 Server is running: http://{}", listener.local_addr()?);

    let app = routes::mount(Router::new(), state.clone())
        .layer(layer)
        .with_state(state);
    serve(listener, app).await?;

    Ok(())
}
