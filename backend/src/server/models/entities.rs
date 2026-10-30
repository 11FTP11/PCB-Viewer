use sqlx::types::Json;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::server::pcb_models::PcbSpec;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub shadow: String,
    pub email: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardsMetadata {
    pub id: i32,
    pub title: String,
    pub public_token: Uuid,
    pub added_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayerMetadata {
    layer_ref: i16,
    visible: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Save {
    pub ref_defs: Vec<String>,
    pub common_side_order: Vec<LayerMetadata>,
    pub mirror_side_order: Vec<LayerMetadata>,
    pub rotation: i16
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardDetail {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub added_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub public_token: Uuid,
    pub save: Option<Json<Save>>,
    pub content: PcbSpec
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoardPublicDetail {
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub updated_at: NaiveDateTime,
    pub public_token: Uuid,
    pub save: Option<Json<Save>>,
    pub content: PcbSpec
}
