use rocket::{http::Status, response::status, serde::json::{json, Json, Value}};
use serde::Deserialize;

use crate::features::{structs::api_response::ApiResponse, users::{middleware::auth::authentication::generate_jwt, model::db::find_users}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login(user: Json<LoginRequest>) -> status::Custom<Json<ApiResponse<Value, String>>> {
    let user = user.into_inner();
    let result = find_users();

    let user_found = result
        .iter()
        .find(|u| u.username == user.username && u.password == user.password);

    if let Some(user_found) = user_found {
        match generate_jwt(&user_found._id) {
            Ok(token) => return status::Custom(
                Status::Ok,
                Json(ApiResponse {
                    code: String::from("features.users.login.success"),
                    message: String::from("user logged success"),
                    data: Some(json!({"user": user_found, "token": token})),
                    args: None
                })
            ),
            Err(err) => return status::Custom(
                Status::NotFound,
                Json(ApiResponse {
                    code: String::from("features.users.login.failed"),
                    message: String::from("fail during generate token"),
                    data: None,
                    args: Some(err.to_string())
                })
            )
        }
    }
    return status::Custom(
        Status::NotFound,
        Json(ApiResponse {
            code: String::from("features.users.login.failed"),
            message: String::from("user not found"),
            data: None,
            args: None
        })
    )
}