mod party;

use axum::{response::IntoResponse, routing::*, Router};

use crate::structures::*;

async fn root() -> impl IntoResponse {
    "Hi"
}

pub fn mount(app: Router<AppState>) -> Router<AppState> {
    app.nest("/party", party::routes()).route("/", get(root))
}
