use actix_web::{web, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, MapErr};

pub async fn route(db: web::Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    let response = HttpResponse::Ok().body("Sign In");

    Ok(response)
}
