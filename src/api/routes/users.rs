use crate::{
    app_error::AppError,
    db::models::user::{unwrap_auth, AuthenticatedUser, User},
};
use actix_web::{
    web::{Data, ReqData},
    HttpResponse,
};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn get_all_users(
    db: Data<Surreal<Client>>,
    auth_user: Option<ReqData<AuthenticatedUser>>,
) -> Result<HttpResponse, AppError> {
    let auth_user = unwrap_auth(auth_user)?;

    let users = User::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(json!({"you": auth_user, "users": users})))
}
