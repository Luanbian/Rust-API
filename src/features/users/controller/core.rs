extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use crate::features::users::model::db::find_users;
use crate::features::structs::api_response::ApiResponse;

#[get("/users")]
pub fn list_users() -> Json<ApiResponse<Value, String>> {
    let result = find_users();
    Json(ApiResponse {
        code: String::from("rust.api.features.list.users.success"),
        message: String::from("list users success"),
        data: Some(json!({"users": result})),
        args: None
    })
}

#[get("/users/<_id>")]
pub fn list_users_by_id(_id: i32) -> Json<ApiResponse<Value, String>> {
    let result = find_users();
    Json(ApiResponse {
        code: String::from("rust.api.features.users.by.id.success"),
        message: String::from("users by id success"),
        data: Some(json!({"users": {"result": result, "id": _id }})),
        args: None
    })
}