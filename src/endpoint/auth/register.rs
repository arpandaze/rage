use crate::core::security::hash_password;
use crate::core::Response;

use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::option::Option;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RegisterData {
    #[validate(length(max = 15))]
    first_name: String,

    #[validate(length(max = 15))]
    middle_name: Option<String>,

    #[validate(length(max = 15))]
    last_name: String,

    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 32))]
    password: String,
}

pub async fn register_endpoint(
    form_data: actix_web::web::Form<RegisterData>,
    db_pool: actix_web::web::Data<PgPool>,
) -> Response<impl Responder> {
    form_data.validate()?;

    sqlx::query!(
        r#"INSERT INTO public.users
        (first_name, middle_name, last_name, email, hashed_password)
        VALUES($1,$2,$3,$4,$5)
    "#,
        form_data.first_name,
        form_data.middle_name,
        form_data.last_name,
        form_data.email,
        hash_password(&form_data.password)?,
    )
    .execute(db_pool.as_ref())
    .await?;

    return Ok(HttpResponse::Ok().finish());
}
