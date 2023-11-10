use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use app_config::AppConfig;
use log;

mod api;
mod app_config;
mod app_error;
mod db;
mod session;
mod utils;
#[macro_use]
mod macros;

use db::db::{ConnectionOptions, DB};

use once_cell::sync::Lazy;
use surrealdb::opt::auth::Root;

static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::init());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    log::info!("Radhey Shyam");
    let namespace = &APP_CONFIG.database_namespace;
    let database = &APP_CONFIG.database_name;
    let username = &APP_CONFIG.database_username;
    let password = &APP_CONFIG.database_password;

    let conn_opts = ConnectionOptions {
        namespace,
        database,
        credentials: Root { username, password },
    };
    let db = DB::connect("127.0.0.1:8000", &conn_opts)
        .await
        .unwrap_or_else(|err| {
            log::error!("Error Connecting To SurrealDB");
            log::error!("{}", err);
            std::process::exit(1);
        });
    log::info!("Connected to SurrealDB...");
    let db_ctx = Data::new(db);

    log::info!("Starting Server at http://localhost:8080");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(db_ctx.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(session::make_session())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5500")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(logger)
            .configure(app_config::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
