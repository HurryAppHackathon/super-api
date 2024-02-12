// #![allow(unused)] // x: remove on production
//! Hurry app hackathon challenge
//! The primary focus of this application revolves around synchronizing view streams among party users.

//! Why using Arc<T> not the smart pointer of T, becuase some types i dont need the extra capacity to mutaute the thing.
#[macro_use]
extern crate serde_with;

#[macro_use]
extern crate lazy_static;

use axum::{
    serve, Router,
};



use socketioxide::{
    SocketIo,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};

use tokio::net::TcpListener;

mod config;
mod error;
mod prelude;

mod extractors;
mod gateway;
mod middlewares;
mod routes;
mod structures;

use config::*;
use prelude::*;

use crate::structures::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let state = AppState {
        parties: Arc::new(Mutex::new(HashMap::new())),
        socket: W(OnceLock::new()),
        users: Arc::new(Mutex::new(vec![])),
        sessions: Arc::new(Mutex::new(vec![])),
    };

    let (layer, io) = SocketIo::builder().with_state(state.clone()).build_layer();

    state.socket.0.set(io.clone()).ok();

    io.ns("/", gateway::on_connect);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT)).await?;

    println!("🚀 Server is running: http://{}", listener.local_addr()?);

    let app = routes::mount(Router::new(), state.clone())
        .layer(layer)
        .with_state(state.clone());
    serve(listener, app).await?;

    Ok(())
}
