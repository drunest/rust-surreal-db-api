use crate::{app_error::AppError, db::models::user::User};
use actix_identity::Identity;
use actix_web::{web::Data, HttpResponse};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn get_all_users(
    db: Data<Surreal<Client>>,
    uid: Option<Identity>,
) -> Result<HttpResponse, AppError> {
    match uid {
        Some(user) => {
            let user_id = user.id()?;

            let users = User::get_all(&db).await?;
            Ok(HttpResponse::Ok().json(json!({"you": user_id, "users": users})))
        }
        None => Err(AppError::UnAuthorized),
    }
}
