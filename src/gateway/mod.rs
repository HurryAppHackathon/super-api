use std::process::Termination;

use axum::{extract::FromRequestParts, http::{request, Request}};
use serde::Deserialize;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef, State as IoState};

use crate::{
    middlewares::{self, verify_user},
    structures::{AppState, Snowflake, User},
};

#[derive(Deserialize)]
struct JoinParty {
    id: String,
}

pub  async fn on_connect(
    socket: SocketRef,
    IoState(state): IoState<AppState>,
) {
    if let Ok(user) = middlewares::auth_parts(socket.req_parts(), state.clone()).await {
        println!("creating listners for {}/{}", user.username, socket.id);
        socket.on(
            "join",
            |_socket: SocketRef, Data::<JoinParty>(data), IoState(state): IoState<AppState>| {
                println!("join request party: {:?}", data.id);
                let parties = state.parties.lock().unwrap();
                if let Some(_party) = parties.get(&Snowflake::try_from(data.id).unwrap()) {
                    println!("sending joined");
                    // TODO: send joined back
                    // socket.(socket.id).emit("joined", party).ok();
                } else {
                    println!("party not found")
                    // FIXME: return something somehow
                }
            },
        );

        socket.on(
            "event",
            |socket: SocketRef,
             IoState(state): IoState<AppState>,
             Data::<Value>(data): Data<Value>| {
                for s in state.socket.sockets().unwrap() {
                    if s.id == socket.id {
                        continue;
                    }
                    socket.to(s.id).emit("event", data.clone()).ok();
                }
            },
        );

        return;
    }

    println!("User not found!");
    socket.disconnect().ok();
}
