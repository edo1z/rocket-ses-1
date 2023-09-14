#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::State;

#[get("/")]
fn index(hoge: &State<String>) -> String {
    format!("Hello, world! {}", hoge)
}

#[post("/send_email_test")]
fn send_email_tets() -> &'static str {
    "Send email test"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
    .manage("hoge".to_string())
    .mount("/", routes![index, send_email_tets])
}
