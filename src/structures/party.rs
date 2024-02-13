use std::sync::Arc;

use serde::Serialize;

use super::*;

#[derive(Clone, Serialize)]
pub struct Party {
    pub name: String,
    pub owner: User,
    pub messages: Vec<Arc<Message>>,
    pub video: Option<Video>,
}
impl Party {
    pub fn new(owner: User, name: &str) -> Self {
        Self {
            name: name.into(),
            owner,
            messages: <_>::default(),
            video: <_>::default(),
        }
    }
}
