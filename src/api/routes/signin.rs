use crate::app_error::AppError;
use actix_web::{web, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn post(db: web::Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().finish())
}
