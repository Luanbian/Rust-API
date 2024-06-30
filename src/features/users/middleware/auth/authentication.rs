use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use crate::features::structs::api_response::ApiResponse;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub subject_id: String,
    pub exp: i64
}

pub fn generate_jwt(_id: &String) -> Result<String, Error> {
    let secret = String::from("asdfghjkl");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: String::from(_id),
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

pub struct JWT {
    pub claims: Claims
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ApiResponse<String, String>;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ApiResponse<String, String>> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
               return Outcome::Error((Status::Unauthorized, ApiResponse {
                code: String::from("users.middleware.auth.failed"),
                message: String::from("No token provided"),
                data: None,
                args: None
               }));
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => {
                    return Outcome::Success(JWT { claims });
                },
                Err(err) => match &err.kind() {
                    ErrorKind::ExpiredSignature => {
                        return Outcome::Error((Status::Unauthorized, ApiResponse {
                            code: String::from("users.middleware.auth.failed"),
                            message: String::from("token expired"),
                            data: None,
                            args: None
                           }));
                    },
                    ErrorKind::InvalidToken => {
                        return Outcome::Error((Status::Unauthorized, ApiResponse {
                            code: String::from("users.middleware.auth.failed"),
                            message: String::from("Invalid token"),
                            data: None,
                            args: None
                           }));
                    }
                    _ => {
                        return Outcome::Error((Status::Unauthorized, ApiResponse {
                            code: String::from("users.middleware.auth.failed"),
                            message: String::from("Fail during validate token"),
                            data: None,
                            args: Some(err.to_string())
                           }));
                    }
                }
            }
        }
    }
}