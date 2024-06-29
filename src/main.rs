mod features;

#[macro_use] extern crate rocket;

use features::users::controller::core::{list_users,list_users_by_id};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            list_users,
            list_users_by_id
        ])
        .launch()
        .await;
}
