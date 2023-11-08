use actix_web::{web, HttpResponse};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, db::models::user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserCreationBody {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email_id: String,
    pub age: u8,
}

pub async fn post(
    body: web::Json<UserCreationBody>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, AppError> {
    let username = body.username.clone();
    let first_name = body.first_name.clone();
    let last_name = body.last_name.clone();
    let password = body.password.clone();
    let email_id = body.email_id.clone();
    let age = body.age;

    let new_user: User = User {
        id: None,
        username,
        first_name,
        last_name,
        email_id,
        password,
        age,
        avatar: None,
    };

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
