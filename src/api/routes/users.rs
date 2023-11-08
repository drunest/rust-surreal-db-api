use actix_web::{web::Data, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, db::models::user::User};

pub async fn get_all_users(db: Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    let users = User::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(users))
}
