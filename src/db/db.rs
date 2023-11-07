use crate::app_error::AppError;
use crate::map_err;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
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
        let db = map_err!(DBErr -> Surreal::new::<Ws>(endpoint).await)?;

        map_err!(DBErr -> db.signin(options.credentials).await)?;

        map_err!(DBErr -> db.use_ns(options.namespace).use_db(options.database).await)?;

        Ok(db)
    }
}

pub trait Creatable: Into<Value> {}
pub trait Updatable: Into<Value> {}
