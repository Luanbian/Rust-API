extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
pub fn list_users() -> Value {
    json!({"users": {"id": "123", "name": "Luan", "age": "22"}})
}