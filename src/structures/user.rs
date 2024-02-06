use std::sync::Arc;

use serde::Serialize;

use super::*;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub username: Arc<str>,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: <_>::default(),
            username: Arc::from(""),
        }
    }
}
impl User {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.into(),
            ..<_>::default()
        }
    }
}
