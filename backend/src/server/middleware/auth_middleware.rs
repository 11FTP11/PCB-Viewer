use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};

use crate::server::ApiError;
use axum_extra::extract::cookie::CookieJar;
use crate::server::jwt;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, ApiError> {
    let jar = CookieJar::from_headers(req.headers());
    if let Some(jwt) = jar.get("token") {
        let token = jwt::extract_from_jwt(jwt.value())?; 
        req.extensions_mut().insert(token);
        Ok(next.run(req).await)
    } else {
        Err(ApiError::Unauthorized)
    }
}