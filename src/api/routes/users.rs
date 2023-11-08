use actix_session::Session;
use actix_web::{web::Data, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, db::models::user::User};

pub async fn get_all_users(
    db: Data<Surreal<Client>>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let curr_user = session.get::<String>("user").map_err(|err| {
        dbg!(err);
        AppError::InternalError("Session Error".into())
    })?;
    dbg!(curr_user);

    let users = User::get_all(&db).await?;

    Ok(HttpResponse::Ok().json(users))
}
