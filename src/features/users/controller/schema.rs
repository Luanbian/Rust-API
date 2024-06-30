use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub password: String
}