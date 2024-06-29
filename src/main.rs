#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Helo world"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}
