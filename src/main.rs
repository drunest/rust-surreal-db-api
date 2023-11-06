use actix_web::{middleware::Logger, App, HttpServer};
use log;

mod api;
mod app_error;
mod db;
mod macros;

use db::db::{ConnectionOptions, DB};
use surrealdb::opt::auth::Root;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let _db = DB::connect("127.0.0.1:8000", &conn_opts).await.unwrap();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
