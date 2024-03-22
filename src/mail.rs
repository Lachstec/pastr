use sendgrid::v3::{Content, Email, Message, Personalization, Sender};
use uuid::Uuid;

pub async fn send_registration_mail(
    user_id: &Uuid,
    mail: &String,
    base_url: &String,
    api_key: &String,
) -> Result<(), anyhow::Error> {
    let p = Personalization::new(Email::new(mail));

    let msg = Message::new(Email::new("pastr@1ux.dev"))
        .set_subject("Pastr Registration")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(format!(
                    r#"Hello,
                    
                    You signed up for pastr, but your account needs to be activated.
                    
                    Simply click on this link: {}/activate?id={}"#,
                    base_url, user_id,
                )),
        )
        .add_personalization(p);

    let sender = Sender::new(api_key.to_owned());
    let response = sender.send(&msg).await?;
    if response.status() != http::StatusCode::ACCEPTED {
        Err(anyhow::anyhow!("error sending mail to user"))
    } else {
        Ok(())
    }
}
