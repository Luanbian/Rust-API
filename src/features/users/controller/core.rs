extern crate rocket;

use rocket::serde::json::{Value, json};
use crate::features::users::model::db::find_users;

#[get("/")]
pub fn list_users() -> Value {
    let result = find_users();
    return json!({"users": result});
}