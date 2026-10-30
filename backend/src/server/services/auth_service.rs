use crate::server::{AuthBody, errors::ServiceError};
use crate::server::jwt;
use bcrypt::{hash, DEFAULT_COST, verify};
use sqlx::PgPool;
use crate::server::models::{ RegisterBody, User  }; 

pub struct AuthService {
    db_con: PgPool,
}

impl AuthService {
    pub fn new(db_con: PgPool) -> Self {
        Self { db_con }
    }

    pub async fn create_user(&self, register_body: RegisterBody) -> Result<(), ServiceError> {
        let predicate = sqlx::query!(
            r#"
                SELECT email FROM users WHERE email = $1
            "#,
            register_body.email
        ).fetch_optional(&self.db_con).await?;

        if predicate.is_some() {
            return Err(ServiceError::EmailIsUsed);
        }

        let hashed = hash(register_body.password.unwrap().as_bytes(), DEFAULT_COST)?;

        sqlx::query!(
            r#"
            INSERT INTO public.users
            (first_name, last_name, shadow, email)
            VALUES ($1, $2, $3, $4)
            "#,
            register_body.first_name,        
            register_body.last_name,         
            hashed,                      
            register_body.email              
        )
        .execute(&self.db_con)
        .await?;
        
        Ok(())
    }

    pub async fn validate_user_data(&self, auth_body: AuthBody) -> Result<(String, String, String), ServiceError> {  
        let candidate = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM public.users WHERE email = $1
            "#,
            &auth_body.email.unwrap()
        )
        .fetch_optional(&self.db_con)
        .await?
        .ok_or(ServiceError::InvalidCredentials)?;

        let mut db_hash = candidate.shadow;
        if db_hash.starts_with("$2a$") {
            db_hash = db_hash.replacen("$2a$", "$2b$", 1);
        }
        
        if verify(auth_body.password.unwrap(), &db_hash)? {
            return Ok((
                jwt::generate_jwt(candidate.id)?,
                candidate.first_name, 
                candidate.last_name
            ));
        }
        
        Err(ServiceError::InvalidCredentials)
    }
}
