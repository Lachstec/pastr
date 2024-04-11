use actix_web_lab::respond::Html;
use askama::Template;

pub mod api;
pub mod healthcheck;
pub mod index;
pub mod user;

#[derive(askama::Template)]
#[template(path = "404.html")]
struct NotFoundPage;

pub async fn not_found() -> Html {
    let page = NotFoundPage.render().unwrap();
    Html(page)
}
