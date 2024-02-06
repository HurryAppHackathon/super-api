use std::sync::Arc;

use serde::Serialize;

use super::*;

#[derive(Default, Clone, Serialize)]
pub struct Party {
    pub name: String,
    pub owner: User,
    pub messages: Vec<Arc<Message>>,
    pub video: Option<Video>,
}
impl Party {
    pub fn new(username: &str, name: &str) -> Self {
        let owner = User::new(username);
        Self {
            name: name.into(),
            owner,
            ..<_>::default()
        }
    }
}
