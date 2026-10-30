use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};
use crate::server::models::entities::BoardsMetadata;

#[derive(Debug, Deserialize, Validate)]
pub struct PaginationMetadata {
    #[validate(range(min = 1, max = 1000, message = "Page must be between 1 and 1000"))]
    pub page: Option<i64>,
    
    #[validate(range(min = 1, max = 100, message = "Per page must be between 1 and 100"))]
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardsPage {
    pub page: i64,
    pub total_pages: i64,
    pub page_data: Vec<BoardsMetadata>
}
