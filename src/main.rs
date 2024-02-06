//! Hurry app hackathon challenge
//! The primary focus of this application revolves around synchronizing view streams among party users.
//! Implementation of an authentication system will be omitted, as it falls outside the scope of this project's objectives.

//! Why using Arc<T> not the smart pointer of T, becuase some types i dont need the extra capacity to mutaute the thing.
use axum::{serve, Router};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{extract::{AckSender, Bin, Data, SocketRef}, SocketIo};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

mod config;
mod error;
mod prelude;

mod routes;
mod structures;

use config::*;
use prelude::*;

use crate::structures::{AppState, Message, User};

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
    
    let (_layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    
    let state = AppState {
        parties: Arc::new(Mutex::new(hash)),
        socket: Arc::new(Mutex::new(io)),
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT)).await?;

    println!("🚀 Server is running: http://{}", listener.local_addr()?);
    

    let app = routes::mount(Router::new()).with_state(state);
    serve(listener, app).await?;

    Ok(())
}
