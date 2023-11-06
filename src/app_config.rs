use actix_web::{get, web, HttpResponse};

use crate::api::routes::{signin, signup};

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Shri Radha")
}
pub fn configure(config: &mut web::ServiceConfig) {
    let api_scope = web::scope("/api")
        .service(web::resource("/auth/signin").route(web::post().to(signin::route)))
        .service(web::resource("/auth/signup").route(web::post().to(signup::route)));

    config.service(hello).service(api_scope);
}
