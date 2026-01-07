use crate::routes;
use crate::middlewares;
use axum::Router;
//use crate::state::AppState;

pub async fn build_app() -> Router {
    //let state = AppState::new();

    Router::new()
        .merge(routes::health::router())
        .merge(routes::projects::router())
        .merge(routes::tasks::router())
        .layer(axum::middleware::from_fn(middlewares::auth::auth_middleware))
        //.with_state(Arc::clone(&state))
}
