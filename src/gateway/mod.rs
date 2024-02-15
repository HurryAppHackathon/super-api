use std::{any::Any, sync::Arc};

use serde::Deserialize;
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef, State as IoState};

use crate::{
    middlewares,
    structures::{AppState, Snowflake},
    Message, Party,
};

#[derive(Deserialize, Clone)]
struct JoinParty {
    pub id: String,
}

#[derive(Deserialize, Clone)]
struct SendMessage {
    pub id: String,
    pub content: String,
}

pub async fn on_connect(socket: SocketRef, IoState(state): IoState<AppState>) {
    if let Ok(user) = middlewares::auth_parts(socket.req_parts(), state.clone()).await {
        println!("creating listners for {}/{}", user.username, socket.id);

        socket.on(
            "message",
            |socket: SocketRef, Data::<SendMessage>(data), IoState(state): IoState<AppState>| {
                let mut parties = state.parties.lock().unwrap();
                if let Some(party) = parties.get_mut(&Snowflake::try_from(data.clone().id).unwrap())
                {
                    let message = Arc::new(Message::new(data.clone().content.into(), user));
                    let messages = [party.messages.clone(), vec![message.clone()]].concat();
                    let party = Arc::new(Party {
                        messages,
                        ..Party::clone(party)
                    });
                    parties.insert(Snowflake::try_from(data.clone().id).unwrap(), party.clone());
                    socket.to(data.id).emit("message", message).ok();
                    println!("{:#?}", party.messages);
                } else {
                    println!("party not found")
                    // FIXME: return something somehow
                }
            },
        );
        socket.on(
            "join",
            |socket: SocketRef, Data::<JoinParty>(data), IoState(state): IoState<AppState>| {
                let parties = state.parties.lock().unwrap();
                if let Some(party) = parties.get(&Snowflake::try_from(data.clone().id).unwrap()) {
                    println!("sending joined {}", socket.id);
                    socket.leave_all().ok();
                    println!("{:#?}", state.socket.sockets().unwrap());
                    socket.join(data.id).ok();
                    socket.emit("joined", party).ok();
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
        #[derive(Deserialize, Clone)]
        struct Event {
            id: String,
            data: Value
        }
        socket.on(
            "event",
            |socket: SocketRef,
             IoState(state): IoState<AppState>,
             Data::<Event>(data): Data<Event>| {
                let parties = state.parties.lock().unwrap();
                if parties.get(&Snowflake::try_from(data.clone().id).unwrap()).is_some() {
                    socket.to(data.id).emit("event", data.data.clone()).ok();
                } else {
                    println!("party not found")
                    // FIXME: return something somehow
                }
            },
        );

        return;
    }

    println!("User not found!");
    socket.disconnect().ok();
}
