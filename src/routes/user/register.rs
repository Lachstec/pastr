use crate::setup::AppBaseUrl;
use actix_web::web;
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "register.html")]
struct RegistrationPage {
    pub base_url: String,
}

pub async fn register(base_url: web::Data<AppBaseUrl>) -> Html {
    let page = RegistrationPage {
        base_url: base_url.0.clone(),
    };
    let html = page.render().unwrap();
    Html(html)
}
