#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
