use crate::app_error::AppError;
use crate::db::models::user::User;
use actix_web::{web, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn post(db: web::Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    let users = User::get_all(db).await?;

    dbg!(&users);

    Ok(HttpResponse::Ok().json(users))
}
