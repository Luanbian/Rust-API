use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub subject_id: String,
    pub exp: i64
}

pub struct JWT {
    pub claims: Claims
}

pub fn generate_jwt(_id: String) -> Result<String, Error> {
    let secret = String::from("asdfghjkl");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: _id,
        exp: expiration
    };
    
    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = String::from("asdfghjkl");
    let token = token.trim_start_matches("Bearer").trim();
    
    match decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(Algorithm::HS512)
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}