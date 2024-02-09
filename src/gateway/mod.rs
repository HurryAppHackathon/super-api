use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};
use socketioxide::extract::{Data, SocketRef, State as IoState};

use crate::{extractors::UserRequest, middlewares::verify_user, structures::AppState};

#[derive(Deserialize, Debug)]
pub struct HandShake {
    token: String,
}

pub fn on_connect(
    socket: SocketRef,
    Data(data): Data<HandShake>,
    IoState(state): IoState<AppState>,
) {
    let token = socket
        .req_parts()
        .headers
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();
    println!("{:?}", state.sessions.lock().unwrap());
    verify_user(token, state);

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
