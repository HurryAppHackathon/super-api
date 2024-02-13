use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}, response::{IntoResponse, Response}};
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

impl User {
    pub fn new(username: &str, hash_password: &str) -> Self {
        Self {
            username: username.into(),
            hash_password: Private::Hidden(hash_password.into()),
            id: <_>::default(),
        }
    }
}



// Extractor for axum
#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if let Some(user) = parts.extensions.get::<User>().cloned() {
            Ok(user)
        } else {
            println!("huh?");
            Err((
                StatusCode::UNAUTHORIZED,
                StatusCode::UNAUTHORIZED.to_string(),
            )
                .into_response())
        }
    }
}
