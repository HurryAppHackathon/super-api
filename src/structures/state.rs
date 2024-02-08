use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use socketioxide::SocketIo;

use super::*;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub sessions: Arc<Mutex<Vec<Session>>>,
    pub parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
    pub socket: Arc<Mutex<SocketIo>>,
}
