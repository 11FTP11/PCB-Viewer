use std::io::Read;

use crate::server::models::entities::{Save, BoardPublicDetail, BoardsMetadata};
use crate::server::models::pagination::BoardsPage;
use crate::server::models::PaginationMetadata;
use crate::server::errors::ServiceError;
use chardetng::EncodingDetector;
use encoding_rs_io::DecodeReaderBytesBuilder;
use sqlx::PgPool;
use uuid::Uuid;
use crate::server::{
    models::entities::BoardDetail,
    pcb_models::PcbSpec,
    PcadLexer,
    PcadParser
};
use axum::body::Bytes;
use chrono::{Local, NaiveDateTime};

use sqlx::types::Json;

pub struct BoardsService {
    db_con: PgPool,
}

impl BoardsService {
    pub fn new(db_con: PgPool) -> Self {
        Self { db_con }
    }

    pub async fn boards_metadata(&self, user_id: i32, pagination: PaginationMetadata) -> Result<BoardsPage, ServiceError> {
        let page = pagination.page.unwrap_or(1);
        let limit = pagination.per_page.unwrap_or(25);
        let offset = (page - 1) * limit;

        let mut current_tc = self.db_con.begin().await?;

        let arr_meta= sqlx::query_as!(
            BoardsMetadata,
            r#"
            SELECT id, title, updated_at, added_at, public_token
            FROM boards_metadata
            WHERE creator_id = $1
            ORDER BY added_at DESC
            LIMIT $2 OFFSET $3 "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&mut *current_tc)
        .await?;
        
        let total_records = sqlx::query_scalar!(
            "SELECT COUNT(id) 
            FROM boards_metadata 
            WHERE creator_id = $1",
            user_id
        )
        .fetch_one(&mut *current_tc)
        .await?
        .unwrap_or_default();

        current_tc.commit().await?;

        let total_pages = (total_records + limit - 1) / limit;

        Ok(
            BoardsPage { 
                page: page, 
                total_pages: total_pages, 
                page_data: arr_meta 
            }
        ) 
        
    }

    pub async fn delete_board(&self, board_id: i32, user_id: i32) -> Result<(), ServiceError> {
        sqlx::query!(
            r#"
                DELETE FROM boards_metadata 
                WHERE id = $1 
                AND creator_id = $2 
            "#,
            board_id,
            user_id
        )
        .execute(&self.db_con)
        .await?;
        
        Ok(())
    }

    
    pub async fn update_board_save(&self, board_id: i32, user_id: i32, save: Save) -> Result<NaiveDateTime, ServiceError> {
        let timestamp = Local::now().naive_local(); 
        let mut current_tc = self.db_con.begin().await?;

        let metadata_result = sqlx::query!(
            r#"
                UPDATE boards_metadata
                SET updated_at = $1
                WHERE id = $2 AND creator_id = $3
            "#,
            timestamp,
            board_id,
            user_id 
        )
        .execute(&mut *current_tc)
        .await?;

        if metadata_result.rows_affected() == 0 {
            return Err(ServiceError::NotFound);
        }

        sqlx::query!(
            r#"
                UPDATE boards_body
                SET save = $1
                WHERE id = $2
            "#,
            Json(save) as Json<Save>, 
            board_id
        )
        .execute(&mut *current_tc) 
        .await?;

        current_tc.commit().await?;

        Ok(timestamp)
    }

    pub async fn public_board_content(&self, board_uuid: Uuid) -> Result<BoardPublicDetail, ServiceError> {
        let raw = sqlx::query!(
            r#"
                SELECT first_name, last_name, title, updated_at, public_token, content, save
                FROM boards_metadata
                JOIN boards_body USING(id)
                JOIN users ON boards_metadata.creator_id = users.id
                WHERE public_token = $1
            "#,
            board_uuid,
        )
        .fetch_optional(&self.db_con)
        .await?
        .ok_or(ServiceError::NotFound)?;
        
        let pcb_spec = serde_json::from_value(raw.content)?;

        Ok(BoardPublicDetail {
            first_name: raw.first_name,
            last_name: raw.last_name,
            title: raw.title,
            updated_at: raw.updated_at,
            content: pcb_spec,
            public_token: raw.public_token,
            save: raw.save.map(serde_json::from_value).transpose()?
        })
    }

    pub async fn board_content(&self, board_id: i32, user_id: i32) -> Result<BoardDetail, ServiceError> {
        let raw = sqlx::query!(
            r#"
                SELECT id, creator_id, title, updated_at, added_at, public_token, content, save
                FROM boards_metadata
                JOIN boards_body USING(id)
                WHERE id = $1 AND creator_id = $2
            "#,
            board_id,
            user_id
        )
        .fetch_optional(&self.db_con)
        .await?
        .ok_or(ServiceError::NotFound)?;
        
        let pcb_spec = serde_json::from_value(raw.content)?;

        Ok(BoardDetail {
            id: raw.id,
            creator_id: raw.creator_id,
            title: raw.title,
            added_at: raw.added_at,
            updated_at: raw.updated_at,
            content: pcb_spec,
            public_token: raw.public_token,
            save: raw.save.map(serde_json::from_value).transpose()?
        })
    }

    pub fn to_utf8(&self, board_bytes: Bytes, buf: &mut String) -> Result<(), ServiceError> {
        let mut decoder = EncodingDetector::new(chardetng::Iso2022JpDetection::Deny);
        decoder.feed(&board_bytes, true);
        let detected_encoding = decoder.guess(None, chardetng::Utf8Detection::Allow);

        let mut decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(detected_encoding))
            .build(&*board_bytes);

        decoder.read_to_string(buf)?;

        Ok(())
    }

    pub async fn load_board(&self, user_id: i32, board_bytes: Bytes, board_title: Option<String>) -> Result<(), ServiceError> {
        let mut decoded_text = String::new();
        self.to_utf8(board_bytes, &mut decoded_text)?;

        let timestamp = Local::now().naive_local(); 
        let pcb_spec = PcadParser::new(
            PcadLexer::new(&decoded_text.as_bytes()))
            .parse_spec()?;

        let mut current_tc = self.db_con.begin().await?;

        let final_title = match board_title {
            Some(ref t) if !t.trim().is_empty() => t.to_string(),
            _ => (timestamp.format("pcb_%Y%m%d_%H%M%S").to_string())
        };

        let metadata_id = sqlx::query!(
            r#"
                INSERT INTO boards_metadata (creator_id, title, added_at, updated_at, public_token)
                VALUES ($1, $2, $3, $4, gen_random_uuid())
                RETURNING id
            "#,
            user_id,
            final_title,
            timestamp,
            timestamp
        )
        .fetch_one(&mut *current_tc)
        .await?
        .id;

        sqlx::query!(
            r#"
                INSERT INTO boards_body (id, content)                
                VALUES ($1, $2)
            "#,
            metadata_id,
            Json(pcb_spec) as Json<PcbSpec>
        )
        .execute(&mut *current_tc)
        .await?;

        current_tc.commit().await?;

        Ok(())
    }

}
