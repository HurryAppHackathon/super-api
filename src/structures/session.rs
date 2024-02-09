use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::Snowflake;
use crate::{prelude::*, SECRET};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: Snowflake,
    pub user_id: Snowflake,
    pub exp: u64,
}

impl Session {
    pub fn new(user_id: Snowflake) -> Self {
        Self {
            id: Snowflake::generate(),
            user_id,
            exp: 10000000000, // later
        }
    }
    pub fn gen_token(&self) -> Result<String> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret((*SECRET).as_ref()),
        )?;
        Ok(token)
    }
    pub fn decode(token: String) -> Result<Session> {
        let token = decode::<Session>(
            &token,
            &DecodingKey::from_secret((*SECRET).as_ref()),
            &Validation::default(),
        )?;

        Ok(token.claims)
    }
}
