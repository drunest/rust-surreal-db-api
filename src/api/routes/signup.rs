use actix_web::{web, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::app_error::AppError;

pub async fn route(db: web::Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    let res = HttpResponse::Ok().body("Sign Up");
    dbg!(db);
    Ok(res)
}
