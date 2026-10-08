use crate::system::middleware::error::MiddlewareError;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;

pub const AUTH_TOKEN: &str = "auth-toker";
pub async fn req_auth<B>(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, MiddlewareError> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    auth_token
        .ok_or_else(|| MiddlewareError::NoAuthToken("No Authentication Token".to_string()))?;
    Ok(next.run(req).await)
}
