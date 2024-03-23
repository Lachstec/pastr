use askama::Template;
use sendgrid::v3::{Content, Email, Message, Personalization, Sender};
use uuid::Uuid;

#[derive(Template, Debug)]
#[template(path = "mail.html")]
#[allow(unused)]
struct SignUpTemplate<'a> {
    user_id: &'a Uuid,
    base_url: &'a str,
}

// TODO: Docs
// TODO: Mail Text
pub async fn send_registration_mail(
    user_id: &Uuid,
    mail: &String,
    base_url: &String,
    api_key: &String,
) -> Result<(), anyhow::Error> {
    let p = Personalization::new(Email::new(mail));
    let template = SignUpTemplate {
        user_id,
        base_url: &base_url,
    };

    let mail_html = template.render()?;

    let msg = Message::new(Email::new("pastr@1ux.dev"))
        .set_subject("Pastr Registration")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(mail_html),
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
