use log::{error, info};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::app_error::AppError;
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
        let db = Surreal::new::<Ws>(endpoint).await.map_err(|err| {
            error!(
                "Something went wrong while connecting to SurrealDB: {}",
                &err
            );
            AppError::DatabaseError(err)
        })?;

        db.signin(options.credentials).await.map_err(|err| {
            error!("Error Authenticating To SurrealDB: {}", &err);
            AppError::DatabaseError(err)
        })?;

        db.use_ns(options.namespace)
            .use_db(options.database)
            .await
            .map_err(|err| {
                error!("Something went wrong connecting to SurrealDB: {}", &err);
                AppError::DatabaseError(err)
            })?;

        info!("Connected to Surreal DB...");

        Ok(db)
    }
}
