use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use dotenvy::dotenv;
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

use surrealdb::opt::auth::Root;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Error Loading Environment Variables");

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    log::info!("Radhey Shyam");
    let conn_opts = ConnectionOptions {
        namespace: "development",
        database: "test",
        credentials: Root {
            username: "radha",
            password: "krsna",
        },
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
            .wrap(session::make_session())
            .configure(app_config::configure)
            .wrap(logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
