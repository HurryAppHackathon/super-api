


use serde::{Deserialize};
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef, State as IoState};

use crate::{
    middlewares::verify_user,
    structures::{AppState, Snowflake},
};

#[derive(Deserialize, Debug)]
pub struct HandShake {
    token: String,
}

#[derive(Deserialize)]
struct JoinParty {
    id: String,
}

pub fn on_connect(
    socket: SocketRef,
    Data(_data): Data<HandShake>,
    IoState(state): IoState<AppState>,
) {
    let token = socket
        .req_parts()
        .headers
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    println!("{token}");
    println!("{:?}", state.sessions.lock().unwrap());

    if let Ok(_user) = verify_user(token, state) {
        println!("creating listners for {}", socket.id);
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
        )
    } else {
        println!("Invalid token ");
    }

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
