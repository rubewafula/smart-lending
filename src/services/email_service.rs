use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::env;
use tracing::{info, error};

pub async fn send_email(to:String, subject:String, body:String) -> &'static str {

    let sender = env::var("ORIGINATOR_EMAIL").unwrap();
    let sender_email_password = env::var("ORIGINATOR_EMAIL_PASSWORD").unwrap();
    let sender_email_host = env::var("ORIGINATOR_EMAIL_HOST").unwrap();
    let sender_email_from = env::var("ORIGINATOR_EMAIL_FROM").unwrap();


    let email = Message::builder()
        .from(format!("{} <{}>", sender_email_from, sender).parse().unwrap())
        .to(to.parse().unwrap())

        .subject(subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    let creds = Credentials::new(sender, sender_email_password);

    // You can replace with your own SMTP server
    let mailer = SmtpTransport::relay(&sender_email_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            info!("Email send success");
            "Email sent successfully!"
        },
        Err(e) => {
            error!("Could not send email: {e:?}");
            "Failed to send email"
        }
    }
}