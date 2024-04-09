use crate::entity::{User, UserError};
use crate::mail;
use crate::routes::api::{ApiErrorMessage, ApiResponse};
use crate::setup::{AppBaseUrl, Pepper, SendGridApiKey};
use actix_web::error::InternalError;
use actix_web::{web, HttpResponse};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    mail: String,
    password: String,
}

#[tracing::instrument(name = "Registration Request", skip(pool, form, sendgrid_key, pepper))]
pub async fn register(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    base_url: web::Data<AppBaseUrl>,
    sendgrid_key: web::Data<SendGridApiKey>,
    pepper: web::Data<Pepper>,
) -> HttpResponse {
    let form_data = form.0;

    let FormData {
        username,
        mail,
        password,
    } = form_data;

    let new_user_id = match User::create(
        &mail,
        &username,
        password,
        &pool,
        pepper.0.expose_secret().as_bytes().to_vec(),
    )
    .await
    {
        Ok(id) => id,
        Err(e) => match e.downcast_ref() {
            Some(UserError::UserAlreadyExists) => {
                return HttpResponse::Conflict().json(ApiResponse::with_errors(
                    false,
                    "user already exists",
                    vec![ApiErrorMessage::user_already_exists()],
                ))
            }
            None => {
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::new(false, "error while processing request"))
            }
        },
    };

    if mail::send_registration_mail(
        &new_user_id,
        &mail,
        &base_url.0,
        sendgrid_key.0.expose_secret(),
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError()
            .json(ApiResponse::new(false, "error while processing request"));
    }

    HttpResponse::Created().json(ApiResponse::new(true, "user registration successful"))
}
