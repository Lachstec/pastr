use crate::{entity::User, setup::AppBaseUrl};
use actix_web::{
    get,
    web::{self, Redirect},
};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/register/activate/{id}")]
pub async fn activate_user(
    user_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
    base_url: web::Data<AppBaseUrl>,
) -> Redirect {
    match User::activate(&user_id, &pool).await {
        Ok(()) => Redirect::to(format!("{}/login?activated=true", base_url.0)).permanent(),
        Err(_) => Redirect::to(format!("{}/login?activated=false", base_url.0)).permanent(),
    }
}
