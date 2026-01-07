use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::http::Request;

pub async fn auth_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    println!("Auth middleware triggered");
    // Placeholder auth middleware
    if req.headers().get("Authorization").is_none() {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    }
    
    next.run(req).await
}