use lettre::{Address, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;

use crate::{log_info, log_warn};

pub fn send() -> String {
    let email_receiver = Mailbox::new(
        Some("zoushihao2022".to_string()),
        Address::new("zoushihao2022", "163.com").unwrap(),
    );
    let mine_email = Mailbox::new(
        Some("VoidWind".to_string()),
        Address::new("voidwind", "qq.com").unwrap(),
    );
    let smtp_server = "smtp.qq.com";
    let password = "iwpyqsddiezbbhae"; //需要生成应用专用密码

    let email = lettre::Message::builder()
        .to(email_receiver)
        .from(mine_email)
        .subject("SUPER NB")
        .header(ContentType::TEXT_HTML)
        .body("<h1>This is a code</h1>\
        <h2>It's a nb code</h2>".to_string())
        .unwrap();

    let creds = Credentials::new(
        "voidwind@qq.com".to_owned(),
        password.to_owned(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

// Send the email
    match mailer.send(&email) {
        Ok(_) => {
            log_info!("Email sent successfully!");
            "Email sent successfully!".to_string()
        }
        Err(e) => {
            log_warn!("Could not send email: {e:?}");
            format!("Could not send email: {e:?}")
        }
    }
}