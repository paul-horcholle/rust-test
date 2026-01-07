use axum::Router;

pub fn router() -> Router {
    Router::new().route("/health", axum::routing::get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}