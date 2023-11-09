use crate::{
    app_error::AppError,
    db::models::user::{SlimUser, User, UserFindableCol},
};
use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Deserialize, Debug)]
pub struct SignInBody {
    pub username: Option<String>,
    pub email_id: Option<String>,
    pub password: String,
}
fn get_auth_cred_method(
    username: Option<String>,
    email_id: Option<String>,
) -> (UserFindableCol, String) {
    match (username, email_id) {
        (Some(username), _) => (UserFindableCol::Username, username),
        (_, Some(email)) => (UserFindableCol::EmailId, email),
        _ => unreachable!(), // You've already checked for both being None above
    }
}
pub async fn login(
    db: web::Data<Surreal<Client>>,
    body: web::Json<SignInBody>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let username = body.username.to_owned();
    let email_id = body.email_id.to_owned();
    let password = body.password.to_owned();

    if username.is_none() && email_id.is_none() {
        return Err(AppError::BadRequest(
            "You need to provide either the 'username' for 'email_id'".into(),
        ));
    }

    let (auth_method, val) = get_auth_cred_method(username, email_id);

    let user = User::find_one(&db, auth_method.clone(), val.clone()).await?;

    match user {
        Some(user) => {
            let check_password = user.verify_password(&password).await?;

            match check_password {
                true => {
                    let slim_user = SlimUser::from(&user);
                    Identity::login(&req.extensions(), slim_user.id.clone()).map_err(|err| {
                        dbg!(err);
                        AppError::InternalError("Something Went Wrong".into())
                    })?;

                    Ok(HttpResponse::Ok().json(json!(
                    {"status": "success",
                    "msg": "logging_in",
                    "user": slim_user,
                    })))
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
