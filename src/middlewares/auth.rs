use axum::middleware::Next;
use axum::response::Response;
use axum::http::Request;

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    // Placeholder auth middleware
    next.run(req).await
}