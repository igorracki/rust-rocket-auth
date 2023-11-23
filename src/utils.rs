use bcrypt::{hash, verify};
use chrono::{DateTime, Utc};
use jsonwebtoken::{Header, Algorithm, EncodingKey, errors::Error, decode, DecodingKey, Validation, TokenData};

use crate::models::Claim;

const JWT_SECRET:&[u8] = b"$3cr3t";

pub fn hash_password(password: &str) -> Result<String, String> {
    return hash(password, 12).map_err(|e| e.to_string());
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match verify(password, hash) {
        Ok(result) => return result,
        Err(_) => return false,
    };
}

pub fn generate_token(username: &str) -> Result<String, String> {
    let expiration: DateTime<Utc> = chrono::Utc::now() + chrono::Duration::seconds(60);
    let claims: Claim = Claim {
        sub: username.to_owned(),
        exp: expiration.timestamp() as usize,
        role: "user".to_owned(),
    };
    let header: Header = Header::new(Algorithm::HS512);
    let token: Result<String, String> = jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).map_err(|e| e.to_string());
    return token;
}

pub fn validate_token(token: &str) -> Result<Claim, String> {
    let token_data: Result<TokenData<Claim>, Error> = decode(token, &DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512));
    match token_data {
        Ok(data) => return Ok(data.claims),
        Err(e) => return Err(format!("Invalid token - {}", e.to_string())),
    };
}
