use serde::Deserialize;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef, State as IoState};

use crate::{
    middlewares::{self},
    structures::{AppState, Snowflake},
};

#[derive(Deserialize)]
struct JoinParty {
    id: String,
}

pub async fn on_connect(socket: SocketRef, IoState(state): IoState<AppState>) {
    if let Ok(user) = middlewares::auth_parts(socket.req_parts(), state.clone()).await {
        println!("creating listners for {}/{}", user.username, socket.id);
        socket.on(
            "join",
            |socket: SocketRef, Data::<JoinParty>(data), IoState(state): IoState<AppState>| {
                println!("join request party: {:?}", data.id);
                let parties = state.parties.lock().unwrap();
                if let Some(party) = parties.get(&Snowflake::try_from(data.id).unwrap()) {
                    println!("sending joined {}", socket.id);
                    socket.emit("joined", party).unwrap();
                } else {
                    println!("party not found")
                    // FIXME: return something somehow
                }
            },
        );
        // event reciver is getting data and transfer it into everyone 
        //
        // for example: 
        //
        // <socket-client>.emit("event", { type: "seek", data: { /* some data about seeking */ } })
        // <socket-client>.emit("event", { type: "pause", data: { None } })
        // <socket-client>.emit("event", { type: "resume", data: { None } })

        socket.on(
            "event",
            |socket: SocketRef,
             IoState(state): IoState<AppState>,
             Data::<Value>(data): Data<Value>| {
                // emit event into everyone except the user that sent the event
                for s in state.socket.sockets().unwrap() {
                    if s.id == socket.id {
                        continue;
                    }
                    s.emit("event", data.clone()).ok();
                }
            },
        );

        return;
    }

    println!("User not found!");
    socket.disconnect().ok();
}
