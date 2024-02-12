use chrono::{DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdGenerator;

use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
lazy_static! {
    // 2024-02-05 21:26:26
    static ref EPOCH: SystemTime = UNIX_EPOCH + Duration::from_millis(1707168386);

    static ref GENERATOR: Mutex<SnowflakeIdGenerator> = Mutex::new(SnowflakeIdGenerator::with_epoch(0, 0, *EPOCH));
}

// #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Snowflake(pub i64);

impl TryFrom<String> for Snowflake {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.parse()?))
    }
}

impl Snowflake {
    pub fn generate() -> Self {
        Self(GENERATOR.lock().unwrap().generate())
    }

    pub fn created_at_timestamp(&self) -> Duration {
        Duration::from_millis((self.0 >> 22) as u64) + EPOCH.duration_since(UNIX_EPOCH).unwrap()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.created_at_timestamp().as_secs() as i64, 0)
            .unwrap()
    }
}

impl Default for Snowflake {
    fn default() -> Self {
        Self::generate()
    }
}
impl ToString for Snowflake {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
