use std::collections::BTreeMap;

use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::Thing,
    sql::{self, Value},
    Surreal,
};

use crate::{app_error::AppError, data_map, map_err};

const TABLE_NAME: &str = "user";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub first_name: String,
    pub last_name: String,
    pub email_id: String,
    pub password: String,
    pub age: u8,
    pub avatar: Option<String>,
}

impl From<User> for Value {
    fn from(user: User) -> Self {
        let mut user_map = data_map![
            "first_name".into() => user.first_name.into(),
            "last_name".into() => user.last_name.into(),
            "email_id".into() => user.email_id.into(),
            "password".into() => user.password.into(),
            "age".into() => user.age.into(),
            "avatar".into() => user.avatar.into(),
        ];

        // Checks if this is a new user or not
        if let Some(id) = user.id {
            user_map.insert("id".into(), id.into());
        }

        user_map.into()
    }
}
impl User {
    pub async fn get_all(db: &Data<Surreal<Client>>) -> Result<Vec<User>, AppError> {
        let q = "SELECT * from type::table($tb);";

        let mut response = map_err!(DBErr -> db.query(q).bind(("tb", TABLE_NAME)).await)?;

        let users = map_err!(DBErr -> response.take::<Vec<User>>(0))?;

        Ok(users)
    }
    // Todo: As this is a User creation add condition to check if the user with the email_id already exits
    /// Creates a new User please only use once
    pub async fn create(&self, db: &Data<Surreal<Client>>) -> Result<Option<User>, AppError> {
        let q = "CREATE type::table($table) CONTENT $data RETURN *";

        let vars =
            data_map!["table".into() => TABLE_NAME.into(), "data".into() => self.clone().into()];

        let mut db_response = map_err!(DBErr -> db.query(q).bind(vars).await)?;

        let user = map_err!(DBErr -> db_response.take::<Option<User>>(0))?;

        dbg!(&user);

        Ok(user)
    }
}
