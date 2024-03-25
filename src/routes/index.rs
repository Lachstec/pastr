use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage;

pub async fn index_page() -> Html {
    // unwrapping is safe here
    let html = IndexPage.render().unwrap();
    Html(html)
}
