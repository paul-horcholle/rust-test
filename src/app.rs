use std::sync::Arc;

use crate::routes;
use axum::Router;
use crate::state::AppState;
// TODO: enable when auth middleware is implemented
// use crate::middlewares;

pub async fn build_app() -> Router {
    let state = AppState::new();

    Router::new()
        .merge(routes::health::router())
        .merge(routes::projects::router())
        .merge(routes::tasks::router())
        // .layer(middlewares::from_fn(auth::auth_middleware))
        .with_state(Arc::clone(&state))
}
