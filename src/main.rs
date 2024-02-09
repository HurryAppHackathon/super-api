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
    cell::OnceCell, collections::HashMap, sync::{Arc, Mutex}
};
use structures::{Session, User};
use tokio::net::TcpListener;

mod config;
mod error;
mod prelude;

mod routes;
mod structures;
mod middlewares;
mod extractors;
mod gateway;

use config::*;
use prelude::*;

use crate::structures::AppState;


#[tokio::main]
async fn main() -> Result<()> {

    let state = AppState {
        parties: Arc::new(Mutex::new(HashMap::new())),
        socket: Arc::new(Mutex::new(OnceCell::new())),
        users: Arc::new(Mutex::new(vec![])),
        sessions: Arc::new(Mutex::new(vec![])),
    };
    dotenv::dotenv().ok();
    let (layer, io) = SocketIo::builder().with_state(state.clone()).build_layer();
    state.socket.lock().unwrap().set(io.clone()).ok();

    io.ns("/", gateway::on_connect);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *PORT)).await?;

    println!("🚀 Server is running: http://{}", listener.local_addr()?);

    let app = routes::mount(Router::new(), state.clone())
        .layer(layer)
        .with_state(state);
    serve(listener, app).await?;

    Ok(())
}
