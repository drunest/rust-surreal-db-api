use crate::{
    app_error::AppError,
    db::models::user::{User, UserFindableCol},
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Deserialize, Debug)]
pub struct SignInBody {
    pub username: Option<String>,
    pub email_id: Option<String>,
    pub password: String,
}

pub async fn post(
    db: web::Data<Surreal<Client>>,
    body: web::Json<SignInBody>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let username = body.username.to_owned();
    let email_id = body.email_id.to_owned();
    let password = body.password.to_owned();

    if username.is_none() && email_id.is_none() {
        return Err(AppError::BadRequest(
            "You need to provide either the 'username' for 'email_id'".into(),
        ));
    }

    let (auth_method, val) = match (username, email_id) {
        (Some(username), _) => (UserFindableCol::Username, username),
        (_, Some(email)) => (UserFindableCol::EmailId, email),
        _ => unreachable!(), // You've already checked for both being None above
    };

    let user = User::find_one(&db, auth_method.clone(), val.clone()).await?;

    match user {
        Some(user) => {
            let check_password = user.verify_password(&password).await?;

            match check_password {
                true => {
                    let th = user.id.unwrap().to_string();
                    session
                        .insert("uid", th)
                        .map_err(|_| AppError::InternalError("Session Error".into()))?;
                    Ok(HttpResponse::Ok().json(json!({"status": "success", "msg": "logging_in" })))
                }
                false => {
                    return Err(AppError::BadRequest("Provided Incorrect Password..".into()));
                }
            }
        }
        None => {
            let method: String = auth_method.clone().into();
            return Err(AppError::BadRequest(format!(
                "User with {} {} not found...",
                method, &val
            )));
        }
    }
}
