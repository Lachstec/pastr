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

/// Send a registration email to the specified mail address.
///
/// Uses the [Sendgrid](https://sendgrid.com/) API to send a confirmation e-mail in order to register.
/// Requires that a valid sendgrid api key is supplied in the config file.
///
/// * `user_id` - uuid of the user that should get activated with this mail
/// * `mail` - mail destination
/// * `base_url` - base url of the service, used to construct correct links in the mail
/// * `api_key` - sendgrid api key
pub async fn send_registration_mail(
    user_id: &Uuid,
    mail: &String,
    base_url: &String,
    api_key: &String,
) -> Result<(), anyhow::Error> {
    let p = Personalization::new(Email::new(mail));
    let template = SignUpTemplate { user_id, base_url };

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
