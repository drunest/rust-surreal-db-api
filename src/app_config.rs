use crate::{
    api::routes::{self, signin, signup},
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

pub fn configure(config: &mut web::ServiceConfig) {
    let custom_json_payload_error =
        web::JsonConfig::default().error_handler(custom_json_error_handler);

    let api_scope = web::scope("/api")
        .service(web::resource("/auth/signin").route(web::post().to(signin::post)))
        .service(web::resource("/auth/signup").route(web::post().to(signup::post)))
        .service(web::resource("/users").route(web::get().to(routes::users::get_all_users)));

    config
        .app_data(custom_json_payload_error)
        .service(hello)
        .service(api_scope);
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub session_secret: String,
}

impl AppConfig {
    pub fn init() -> Self {
        dotenv().expect("Error Loading Environment Variables");
        let session_secret = std::env::var("SECRET").unwrap();

        AppConfig { session_secret }
    }
}
