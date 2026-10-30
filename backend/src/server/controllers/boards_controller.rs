use std::sync::Arc;
use crate::server::{models::{Claims, PaginationMetadata, entities::Save}, services::BoardsService};
use axum::{
    Extension, 
    Json, 
    extract::{Multipart, Path, State}, 
    http::StatusCode, 
    response::IntoResponse
};
use axum::extract::Query;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::server::errors::ApiError;

#[derive(Clone)]
pub struct BoardsController {
    service: Arc<BoardsService>,  
}

impl BoardsController {
    pub fn new(service: BoardsService) -> Self {
        Self {
            service: Arc::new(service),  
        }
    }

    pub async fn user_boards(
        Extension(jwt_data): Extension<Claims>,
        State(class_instance): State<Self>,
        Query(pagination): Query<PaginationMetadata>
    ) -> Result<impl IntoResponse, ApiError> {
        pagination.validate()?;
        
        let data = class_instance.service.boards_metadata(jwt_data.sub, pagination).await?;  

        Ok((
            StatusCode::OK,  
            Json(json!({
                "code": "boards_metadata_retrieved",
                "data": data
            }))
        ))
    }

    pub async fn delete_board(
        Extension(jwt_data): Extension<Claims>,
        State(class_instance): State<Self>,
        Path(board_id): Path<i32>
    ) -> Result<impl IntoResponse, ApiError> {

        class_instance.service.delete_board(board_id, jwt_data.sub).await?;  

        Ok((
            StatusCode::CREATED,  
            Json(json!({
                "code": "board_deleted",
                "message": "Board has been deleted successfully"
            }))
        ))
    }

    pub async fn provide_board(
        Extension(jwt_data): Extension<Claims>,
        State(class_instance): State<Self>,
        Path(board_id): Path<i32>
    ) -> Result<impl IntoResponse, ApiError> {

        let content = class_instance.service.board_content(board_id, jwt_data.sub).await?;  
        Ok((
            StatusCode::OK,  
            Json(json!({
                "code": "board_content_provided",
                "data": content 
            }))
        ))
    }

    pub async fn provide_public_board(
        State(class_instance): State<Self>,
        Path(board_uuid): Path<Uuid>
    ) -> Result<impl IntoResponse, ApiError> {

        let content = class_instance.service.public_board_content(board_uuid).await?;  
        Ok((
            StatusCode::OK,  
            Json(json!({
                "code": "board_content_provided",
                "data": content 
            }))
        ))
    }
    
    pub async fn update_save(
        Extension(jwt_data): Extension<Claims>,
        State(class_instance): State<Self>,
        Path(board_id): Path<i32>,
        Json(save): Json<Save>
    ) -> Result<impl IntoResponse, ApiError> {
        let updated_date = class_instance.service.update_board_save(board_id, jwt_data.sub, save).await?;
        Ok((
            StatusCode::OK,  
            Json(json!({
                "code": "board_save_updated",
                "updatedAt": updated_date
            }))
        ))
    }

    pub async fn add_board(
        Extension(jwt_data): Extension<Claims>,
        State(class_instance): State<Self>,
        mut multipart: Multipart
    ) -> Result<impl IntoResponse, ApiError> {

        let mut file_bytes = None;
        let mut title = None;


        while let Some(field) = multipart.next_field().await? {
            match field.name() {
                Some("file") => {
                    if file_bytes.is_some() {
                        return Err(ApiError::BadRequest("Multiple files not allowed"));
                    }
                    file_bytes = Some(field.bytes().await?);
                }
                Some("title") => {
                    if title.is_some() {
                        return Err(ApiError::BadRequest("Multiple files not allowed"));
                    }
                    title = Some(field.text().await?);
                }
                _ => {}
            }

            if file_bytes.is_some() && title.is_some() { 
                break; 
            }
        }

        let board_bytes = file_bytes.ok_or(ApiError::BadRequest("File is required"))?;

        class_instance.service.load_board(jwt_data.sub, board_bytes, title).await?;  
        Ok((
            StatusCode::OK,  
            Json(json!({
                "code": "board_added",
                "messsage": "board added successfully" 
            }))
        ))
    }
}
