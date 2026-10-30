use crate::server::{
    errors::ServiceError,
    constants::JWT_SECRET_KEY,
    models::token::*,
};

use crate::server::models::{Claims, auth};
use jsonwebtoken::{decode, encode, Algorithm, Header, DecodingKey, EncodingKey, Validation, };
use std::time::{SystemTime, UNIX_EPOCH};



pub fn get_secret() -> &'static [u8] {
    &JWT_SECRET_KEY
}

pub fn generate_jwt(user_id: i32) -> Result<String, ServiceError> {
    let issued = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let auth_token =  Claims {
        exp: issued + 86400 * 30,
        iat: issued,
        sub: user_id,
    };

    Ok(encode(&Header::default(), &auth_token, &EncodingKey::from_secret(get_secret()))?)
}

pub fn extract_from_jwt(jwt: &str) -> Result<Claims, ServiceError> {  
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;  

    Ok(decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(get_secret()),
        &validation
    )?.claims)
}
