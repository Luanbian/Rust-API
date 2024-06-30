mod features;

#[macro_use] extern crate rocket;
use rocket::serde::json::{Value, json};
use features::users::controller::{core::{list_users,list_users_by_id}, login::login};


#[catch(404)]
pub fn route_not_found() -> Value {
    json!("Not found")
}

#[catch(500)]
pub fn internal_server_error() -> Value {
    json!("Internal server error")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            list_users,
            list_users_by_id,
            login
        ])
        .register("/", catchers![
            route_not_found,
            internal_server_error
        ])
        .launch()
        .await;
}
