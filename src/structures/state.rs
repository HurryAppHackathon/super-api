use std::{
    cell::OnceCell, collections::HashMap, ops::Deref, sync::{Arc, Mutex}
};

use socketioxide::SocketIo;

use crate::prelude::*;
use super::*;

struct MySocket {
    socket: OnceCell<SocketIo>
}

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub sessions: Arc<Mutex<Vec<Session>>>,
    pub parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
    pub socket: Arc<Mutex<W<OnceCell<SocketIo>>>>,
}


impl Deref for W<OnceCell<SocketIo>>  {
    type Target = SocketIo;

    fn deref(&self) -> &Self::Target {
            self.0.get().unwrap()
    }
}