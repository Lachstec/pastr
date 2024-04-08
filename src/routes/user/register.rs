use actix_web::HttpResponse;
use actix_web::{error::InternalError, web};
use actix_web_lab::respond::Html;
use askama::Template;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use thiserror::Error;

use crate::entity::User;
use crate::mail;
use crate::setup::{AppBaseUrl, Pepper, SendGridApiKey};

#[derive(Template)]
#[template(path = "register.html")]
struct RegistrationPage;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    mail: String,
    password: String,
}

#[derive(Debug, Error)]
pub enum RegistrationError {
    #[error("failed to send activation mail")]
    MailError(#[source] anyhow::Error),
    #[error("invalid user data")]
    InvalidDataError(#[source] anyhow::Error),
}

#[tracing::instrument(name = "Registration Request", skip(pool, form, sendgrid_key, pepper))]
pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    base_url: web::Data<AppBaseUrl>,
    sendgrid_key: web::Data<SendGridApiKey>,
    pepper: web::Data<Pepper>,
) -> Result<HttpResponse, InternalError<RegistrationError>> {
    let form_data = form.0;

    let FormData {
        username,
        mail,
        password,
    } = form_data;

    let new_user_id = User::create(
        &mail,
        &username,
        password,
        &pool,
        pepper.0.expose_secret().as_bytes().to_vec(),
    )
    .await
    .map_err(|e| {
        println!("{}", e);
        InternalError::from_response(
            RegistrationError::InvalidDataError(e),
            HttpResponse::BadRequest().finish(),
        )
    })?;

    mail::send_registration_mail(
        &new_user_id,
        &mail,
        &base_url.0,
        sendgrid_key.0.expose_secret(),
    )
    .await
    .map_err(|e| {
        println!("{:?}", e);
        InternalError::from_response(
            RegistrationError::MailError(e),
            HttpResponse::InternalServerError().finish(),
        )
    })?;

    Ok(HttpResponse::Created().finish())
}

pub async fn register_page() -> Html {
    let html = RegistrationPage.render().unwrap();
    Html(html)
}
