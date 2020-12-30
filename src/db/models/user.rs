use std::result;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::Insertable;
use diesel::{insert_into, RunQueryDsl};
use log::{error, info};

use crate::bot::dialogs::Command;
use crate::db::models::dialog::DialogEntity;
use crate::db::schema::users::columns::id as id_column;
use crate::db::schema::users::dsl::currency;
use crate::db::Connection;

use crate::db::dialogs as dialogs_table;
use crate::db::schema::users;
use crate::db::users as users_table;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserEntity {
    pub id: String,
    pub currency: Option<String>,
}

type Result<T> = result::Result<T, Error>;

impl UserEntity {
    pub fn new(id: String) -> Self {
        UserEntity { id, currency: None }
    }

    pub fn get_users(conn: &Connection) -> Result<Vec<UserEntity>> {
        match users_table.load::<UserEntity>(conn) {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("failed to get users: {}", err);
                Err(err)
            }
        }
    }

    pub fn save_user(user_id: &str, conn: &Connection) -> Result<UserEntity> {
        let new_user = UserEntity::new(user_id.to_string());
        info!("creating new user: {:?}", new_user);

        let statement = insert_into(users_table).values(&new_user);
        let result = statement.execute(conn).map(|_| {
            diesel::insert_into(dialogs_table)
                .values(DialogEntity::new(
                    user_id.to_string(),
                    Command::Start.to_string(),
                    Some("CurrencySelection".to_string()),
                ))
                .execute(conn)
        });
        match result {
            Ok(_) => Ok(new_user),
            Err(err) => {
                error!("failed to create new user: {}", err);
                Err(err)
            }
        }
    }

    pub fn is_registered(user_id: &str, conn: &Connection) -> Result<bool> {
        info!("Check if user {} registered", user_id);
        match users_table
            .filter(id_column.eq(user_id))
            .load::<UserEntity>(conn)
        {
            Ok(result) => Ok(!result.is_empty()),
            Err(err) => {
                error!("failed to check if user {} registered", err);
                Err(err)
            }
        }
    }

    pub fn update_currency(new_currency: &str, user_id: &str, conn: &Connection) -> Result<usize> {
        info!("currency {} update for user: {}", new_currency, user_id);
        let target = users_table.filter(id_column.eq(user_id));
        match diesel::update(target)
            .set(currency.eq(new_currency))
            .execute(conn)
        {
            Ok(affected) => Ok(affected),
            Err(err) => {
                error!("failed to update currency for user: {}, {}", user_id, err);
                Err(err)
            }
        }
    }
}
