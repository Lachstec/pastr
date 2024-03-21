use actix_web::{http::header::ContentType, HttpResponse};

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<h1>Hello, World!</h1>")
}
