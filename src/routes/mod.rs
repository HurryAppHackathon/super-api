mod auth;
mod party;

use axum::{
    middleware::{self},
    response::IntoResponse,
    routing::*,
    Router,
};

use crate::{middlewares, structures::*};

async fn root() -> impl IntoResponse {
    "Hi"
}

pub fn mount(app: Router<AppState>, state: AppState) -> Router<AppState> {
    // im hating this
    app
    .route("/", get(root))
        .nest("/auth", auth::routes())
        .nest(
            "/party",
            party::routes().layer(middleware::from_fn_with_state(state, middlewares::auth)),
        )
}
