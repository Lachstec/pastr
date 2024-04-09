use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "register.html")]
struct RegistrationPage;

pub async fn register() -> Html {
    let html = RegistrationPage.render().unwrap();
    Html(html)
}
