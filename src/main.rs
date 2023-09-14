#[macro_use]
extern crate rocket;

pub mod email;

use crate::email::{init_ses, send_email, EmailParams, SesClient};
use dotenv::dotenv;
use rocket::State;

#[post("/send_email_test")]
async fn send_email_test(client: &State<SesClient>) -> Result<(), String> {
    let params = EmailParams {
        to: "hoge@example.com".to_string(),
        from: "from@example.com".to_string(),
        subject: "test".to_string(),
        body: "Hello, World!".to_string(),
    };

    send_email(client, params)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .manage(init_ses().await)
        .mount("/", routes![send_email_test])
}
