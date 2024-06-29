mod features;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![features::users::controller::core::list_users])
        .launch()
        .await;
}
