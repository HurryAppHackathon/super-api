use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use socketioxide::SocketIo;

use super::*;

#[derive(Clone)]
pub struct AppState {
    pub parties: Arc<Mutex<HashMap<Snowflake, Arc<Party>>>>,
    pub socket: Arc<Mutex<SocketIo>>,
}
