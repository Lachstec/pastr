use crate::entity::User;
use actix_web::{
    web::{self, Redirect},
    Either,
};
use actix_web_lab::respond::Html;
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "activated.html")]
struct ActivationPage;

#[derive(serde::Deserialize, Debug)]
pub struct ActivationQuery {
    pub user_id: Option<Uuid>,
}

#[tracing::instrument(name = "Account Activation Request", skip(pool))]
pub async fn activate_user(
    user_id: web::Query<ActivationQuery>,
    pool: web::Data<PgPool>,
) -> Either<Html, Redirect> {
    match user_id.0.user_id {
        Some(id) => match User::activate(&id, &pool).await {
            Ok(_) => Either::Left(Html(ActivationPage.render().unwrap())),
            Err(_) => Either::Right(Redirect::to("/notfound")),
        },
        None => Either::Right(Redirect::to("/notfound")),
    }
}
