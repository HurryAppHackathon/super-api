mod auth;
mod party;

use axum::{async_trait, extract::{FromRequest, Request, State}, http::StatusCode, middleware::{self, Next}, response::{IntoResponse, Response}, routing::*, Router};

use crate::{middlewares, structures::*};

async fn root() -> impl IntoResponse {
    "Hi"
}

pub fn mount(app: Router<AppState>, state: AppState) -> Router<AppState> {
    app.route("/", get(root))
        .nest("/auth", auth::routes())
        .nest("/party", party::routes().layer(middleware::from_fn_with_state(state , middlewares::auth)))
}



