use actix_web::{web, HttpResponse};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, db::models::user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserCreation {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email_id: String,
}

pub async fn post(db: web::Data<Surreal<Client>>) -> Result<HttpResponse, AppError> {
    return Ok(HttpResponse::Ok().finish());
    let new_user: User = todo!();

    let new_user = new_user.create(&db).await?;

    match new_user {
        Some(user) => {
            let response_body = json!({
                "status": 201,
                "message": "User Created",
                "user": user,
            });
            Ok(HttpResponse::Created().json(response_body))
        }
        None => Ok(HttpResponse::Ok().finish()),
    }
}
