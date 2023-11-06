use crate::app_error::AppError;
use crate::MapErr;
use log;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub struct DB;
pub struct ConnectionOptions<'a> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub credentials: Root<'a>,
}

impl DB {
    pub async fn connect<'a>(
        endpoint: &'static str,
        options: &ConnectionOptions<'a>,
    ) -> Result<Surreal<Client>, AppError> {
        let db = MapErr!(DBError -> Surreal::new::<Ws>(endpoint).await)?;

        MapErr!(DBError -> db.signin(options.credentials).await)?;

        MapErr!(DBError -> db.use_ns(options.namespace).use_db(options.database).await)?;

        Ok(db)
    }
}
