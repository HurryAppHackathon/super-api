mod auth;
mod party;

use axum::{response::IntoResponse, routing::*, Router};

use crate::structures::*;

async fn root() -> impl IntoResponse {
    "Hi"
}

pub fn mount(app: Router<AppState>) -> Router<AppState> {
    app.route("/", get(root))
        .nest("/auth", auth::routes())
        .nest("/party", party::routes())
}
