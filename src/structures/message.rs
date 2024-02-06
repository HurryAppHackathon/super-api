use std::sync::Arc;

use serde::Serialize;

use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub id: Snowflake,
    pub content: Arc<str>,
    pub author: User,
}
impl Message {
    pub fn new(content: Arc<str>, author: User) -> Self {
        Self {
            id: Snowflake::generate(),
            content,
            author,
        }
    }
}
impl Default for Message {
    fn default() -> Self {
        Self {
            content: Arc::from(""),
            id: <_>::default(),
            author: <_>::default(),
        }
    }
}
