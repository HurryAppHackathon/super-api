use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Private<T> {
    Hidden(T),
    Shown(T),
}

impl<T: Default> Default for Private<T> {
    fn default() -> Self {
        Self::Hidden(T::default())
    }
}
impl<T> From<T> for Private<T> {
    fn from(t: T) -> Self {
        Self::Hidden(t)
    }
}

impl<T: Clone> Private<T> {
    pub fn is_private(&self) -> bool {
        matches!(self, Private::Hidden(_))
    }

    pub fn set_public(&mut self) {
        *self = Self::Shown((**self).clone());
    }

    pub fn set_private(&mut self) {
        *self = Self::Hidden((**self).clone());
    }
}

impl<T> std::ops::Deref for Private<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Hidden(x) | Self::Shown(x) => x,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde_as]
pub struct User {
    pub id: Snowflake,
    pub username: Arc<str>,
    #[serde(skip_serializing_if = "Private::is_private")]
    pub hash_password: Private<Arc<str>>,
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: <_>::default(),
            username: Arc::from(""),
            hash_password: Private::Hidden(Arc::from("")),
        }
    }
}
impl User {
    pub fn new(username: &str, hash_password: &str) -> Self {
        Self {
            username: username.into(),
            hash_password: Private::Hidden(hash_password.into()),
            ..<_>::default()
        }
    }
}
