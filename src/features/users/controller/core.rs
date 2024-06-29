extern crate rocket;

use rocket::serde::json::{Value, json};
use crate::features::users::model::db::find_users;

#[get("/users")]
pub fn list_users() -> Value {
    let result = find_users();
    return json!({"users": result});
}

#[get("/users/<_id>")]
pub fn list_users_by_id(_id: i32) -> Value {
    let result = find_users();
    return json!({"users": {"result": result, "id": _id }});
}