extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use crate::features::users::middleware::auth::authentication::JWT;
use crate::features::users::model::db::find_users;
use crate::features::structs::api_response::ApiResponse;
use super::schema::User;

#[get("/users")]
pub fn list_users() -> status::Custom<Json<ApiResponse<Value, User>>> {
    let result = find_users();
    if result.is_empty() {
        return status::Custom(
            Status::NoContent,
            Json(ApiResponse {
                code: String::from("rust.api.features.list.users.no.content"),
                message: String::from("list users no content"),
                data: None,
                args: None
            })
        );
    }
    return status::Custom(
        Status::Ok,
        Json(ApiResponse {
            code: String::from("rust.api.features.list.users.success"),
            message: String::from("list users success"),
            data: Some(json!({"users": result})),
            args: None
        })
    );
}

#[get("/users/<_id>")]
pub fn list_users_by_id(_id: i32, _auth: JWT) -> status::Custom<Json<ApiResponse<Value, User>>> {
    let result = find_users();
    if result.is_empty() {
        return status::Custom(
            Status::NotFound,
            Json(ApiResponse {
                code: String::from("rust.api.features.users.by.id.fail"),
                message: format!("user with id {} not found", _id),
                data: None,
                args: None
            })
        );
    }
    return status::Custom(
        Status::Ok,
        Json(ApiResponse {
            code: String::from("rust.api.features.users.by.id.success"),
            message: String::from("list user by id success"),
            data: Some(json!({"users": {"result": result, "id": _id }})),
            args: None
        })
    );
}