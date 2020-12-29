use std::result;

use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::Insertable;
use log::{error, info};

use crate::db::history as history_table;
use crate::db::schema::history;
use crate::db::Connection;

type Result<T> = result::Result<T, Error>;

#[derive(Insertable)]
#[table_name = "history"]
pub struct HistoryEntity {
    pub id: i32,
    pub user_id: String,
    pub amount: f32,
    pub category: Option<String>,
    pub created: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "history"]
pub struct HistoryPatch {
    pub user_id: String,
    pub amount: f32,
    pub category: Option<String>,
    pub created: NaiveDateTime,
}

impl HistoryPatch {
    pub fn new(user_id: String, amount: f32) -> Self {
        HistoryPatch {
            user_id,
            amount,
            created: Utc::now().naive_utc(),
            category: None,
        }
    }
}

impl HistoryEntity {
    pub fn add_expense_record(user_id: String, amount: f32, conn: &Connection) -> Result<()> {
        info!(
            "inserting expense record for user {} with amount {}",
            user_id, amount
        );
        match insert_into(history_table)
            .values(HistoryPatch::new(user_id, amount))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to create new user: {}", err);
                Err(err)
            }
        }
    }
}
