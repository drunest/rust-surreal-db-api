use crate::{
    api::{
        middlewares::require_auth::RequireAuthentication,
        routes::{self, signin, signup},
    },
    app_error::AppError,
};
use actix_web::{
    error::JsonPayloadError,
    get,
    web::{self},
    HttpRequest, HttpResponse,
};
use dotenvy::dotenv;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Shri Radha")
}

// Fn(JsonPayloadError, &HttpRequest) -> Error + Send + Sync + 'static

fn custom_json_error_handler(err: JsonPayloadError, req: &HttpRequest) -> actix_web::Error {
    match err {
        JsonPayloadError::ContentType => AppError::BadRequest(format!(
            "Content Type Should be 'application/json', at {}",
            req.path()
        ))
        .into(),
        JsonPayloadError::Deserialize(_) => AppError::BadRequest(format!(
            "The request body to this route {} is not accurate. Please refer to the docs",
            req.path()
        ))
        .into(),
        _ => AppError::BadRequest(format!("{}", err)).into(),
    }
}
// ! Routing
//? - /
//?- /api
//?     - /auth
//?         - /login
//?         - /register
//?
//?     - /v1 -> **TODO!**
//?     - /admin *admin_only*
//?         - /users
/// Configures the app routing and errors
pub fn configure(config: &mut web::ServiceConfig) {
    // Set up custom JSON payload error handler
    let custom_json_payload_error =
        web::JsonConfig::default().error_handler(custom_json_error_handler);

    // Routes for Authentication
    let auth = web::scope("/auth")
        .service(web::resource("/login").route(web::post().to(signin::login)))
        .service(web::resource("/register").route(web::post().to(signup::register)));

    // Define v1 version routes
    let v1 = web::scope("/v1");

    // Define admin only routes
    let admin = web::scope("/admin")
        .wrap(RequireAuthentication::default().set_admin_only(true))
        .service(web::resource("/users").route(web::get().to(routes::users::get_all_users)));

    // Create API scope containing authentication and version 1 routes
    let api_scope = web::scope("/api").service(auth).service(v1).service(admin);

    config
        .app_data(custom_json_payload_error)
        .service(hello)
        .service(api_scope);
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub session_secret: String,
    pub database_name: String,
    pub database_namespace: String,
    pub database_username: String,
    pub database_password: String,
    pub auth_cookie_key: String,
}

impl AppConfig {
    /// Initializes dotenv and returns a `AppConfig` containing all the required variables
    pub fn init() -> Self {
        dotenv().expect("Error Loading Environment Variables");
        let session_secret = get_env("SESSION_SECRET");
        let database_namespace = get_env("SURREAL_NAMESPACE");
        let database_name = get_env("SURREAL_DATABASE");
        let database_username = get_env("SURREAL_USERNAME");
        let database_password = get_env("SURREAL_PASSWORD");
        let auth_cookie_key = get_env("AUTH_COOKIE_NAME");
        AppConfig {
            session_secret,
            database_name,
            database_namespace,
            database_password,
            database_username,
            auth_cookie_key,
        }
    }
}

/// Gets a env variable from the environment panics if not loaded
fn get_env(key: &str) -> String {
    std::env::var(key).expect(&format!("ENVIRONMENT ERROR: {} not set!", key))
}
