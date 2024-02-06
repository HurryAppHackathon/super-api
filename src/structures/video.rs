use std::sync::Arc;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Video {
    pub video_url: Arc<str>,
}
impl Video {
    pub fn new(video_url: &str) -> Self {
        Self {
            video_url: video_url.into(),
        }
    }
}
impl Default for Video {
    fn default() -> Self {
        Self {
            video_url: Arc::from(""),
        }
    }
}
