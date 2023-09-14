use aws_config;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
pub use aws_sdk_sesv2::Client as SesClient;

pub struct EmailParams {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

pub async fn init_ses() -> SesClient {
    let config = aws_config::load_from_env().await;
    let client = SesClient::new(&config);

    client
}

pub async fn send_email(client: &SesClient, params: EmailParams) -> Result<(), String> {
    let mut dest = Destination::builder().build();
    dest.to_addresses = Some(vec![params.to]);

    let subject_content = Content::builder()
        .data(params.subject)
        .charset("UTF-8")
        .build();

    let body_content = Content::builder()
        .data(params.body)
        .charset("UTF-8")
        .build();

    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    client
        .send_email()
        .from_email_address(params.from)
        .destination(dest)
        .content(email_content)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
