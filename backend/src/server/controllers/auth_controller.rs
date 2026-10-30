use crate::server::models::{AuthBody, RegisterBody};
use axum::{
    Json,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    extract::State,
};
use serde_json::json;
use validator::Validate;
use crate::server::errors::ApiError;
use crate::server::services::AuthService;
use std::sync::Arc;
use crate::server::constants::MASTER_PASSWORD;

#[derive(Clone)]
pub struct AuthController {
    service: Arc<AuthService>,  
}

impl AuthController {
    pub fn new(service: AuthService) -> Self {
        Self {
            service: Arc::new(service),  
        }
    }

    pub async fn register(
        State(class_instance): State<Self>,
        headers: HeaderMap,
        Json(payload): Json<RegisterBody>,
    ) -> Result<impl IntoResponse, ApiError> {

        let auth_header = headers.get("Authorization")
            .ok_or(ApiError::BadRequest("Authorization header required"))?;


        if auth_header.to_str()? != format!("Bearer {}", *MASTER_PASSWORD)  {
            return Err(ApiError::BadRequest("Invalid master key"));
        }

        payload.validate()?;

        class_instance.service.create_user(payload).await?;  

        Ok(
            (StatusCode::CREATED,  
            Json(json!({
                "code": "user_created",
                "message": "User has been created successfully",
            }))
        ))
    }

    pub async fn logout() -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.append(
            "Set-Cookie",
            HeaderValue::from_static("token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax")
        );

        headers.append(
            "Set-Cookie",
            HeaderValue::from_static("userInfo=; Path=/; Max-Age=0; SameSite=Lax")
        );

        (
            headers,
            (StatusCode::OK,
                Json(json!({
                    "code": "logout_provided",
                    "message": "Delete token provided",
                })))
        )
    }

    pub async fn login(
        State(class_instance): State<Self>,
        Json(payload): Json<AuthBody>,  
    ) -> Result<impl IntoResponse, ApiError> {
        payload.validate()?;
        let data = class_instance.service.validate_user_data(payload).await?;  
        let mut headers = HeaderMap::new();
        headers.append(
            "Set-Cookie",
            HeaderValue::from_str(&format!("token={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Lax", data.0))?
        );

        let full_name = format!("{} {}", data.1, data.2);

        headers.append(
            "Set-Cookie",
            HeaderValue::from_str(&format!("userInfo={}; Path=/; Max-Age=86400 SameSite=Lax", urlencoding::encode(&full_name)))?
        );

        Ok((
            headers,
            (StatusCode::OK,
                Json(json!({
                    "code": "login_success",
                    "message": "Login successful",
                })))
        ))
    }
}
