use std::{
    cell::OnceCell,
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex, OnceLock},
};

use socketioxide::SocketIo;

use super::*;
use crate::prelude::*;


#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub sessions: Arc<Mutex<Vec<Session>>>,
    pub parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
    pub socket: W<OnceLock<SocketIo>>,
}

impl Deref for W<OnceLock<SocketIo>> {
    type Target = SocketIo;

    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}
