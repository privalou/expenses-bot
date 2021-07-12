use std::result;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::Insertable;
use diesel::{insert_into, update};
use log::{error, info};

use crate::db::history as history_table;
use crate::db::schema::{
    history,
    history::columns::{created as created_at_column, user_id as user_id_column},
};
use crate::db::Connection;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Identifiable, Queryable)]
#[table_name = "history"]
pub struct HistoryEntity {
    pub id: i32,
    pub user_id: String,
    pub amount: f32,
    pub category: Option<String>,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "history"]
pub struct NewHistoryRecord {
    pub user_id: String,
    pub amount: f32,
    pub category: Option<String>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "history"]
pub struct HistoryPatch {
    pub amount: Option<f32>,
    pub category: Option<String>,
    pub updated: NaiveDateTime,
}

impl NewHistoryRecord {
    pub fn new(user_id: String, amount: f32, category: Option<String>) -> Self {
        NewHistoryRecord {
            user_id,
            amount,
            category,
            created: Utc::now().naive_utc(),
        }
    }
}

impl HistoryPatch {
    pub fn new(amount: Option<f32>, category: Option<String>) -> Self {
        HistoryPatch {
            amount,
            category,
            updated: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HistoryRepository {}

impl HistoryRepository {
    pub fn add_expense_record(user_id: String, amount: f32, conn: &Connection) -> Result<()> {
        info!(
            "inserting expense record for user {} with amount {}",
            user_id, amount
        );
        match insert_into(history_table)
            .values(NewHistoryRecord::new(user_id, amount, None))
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to create new user: {}", err);
                Err(err)
            }
        }
    }

    pub fn update_latest_expense_record(
        user_id: String,
        history_patch: &HistoryPatch,
        conn: &Connection,
    ) -> Result<()> {
        info!(
            "updating latest history record for user {} with patch {:?}",
            user_id, history_patch
        );
        let target = history_table
            .order(created_at_column.desc())
            .filter(user_id_column.eq(user_id))
            .first::<HistoryEntity>(conn)
            .expect("No such dialog");
        info!("retrieved latest record {:?}", &target);
        match update(&target).set(history_patch).execute(conn) {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to update latest record: {}", err);
                Err(err)
            }
        }
    }

    pub fn get_all_records(user_id: String, conn: &Connection) -> Result<Vec<HistoryEntity>> {
        info!("retrieving records for user {}", user_id,);
        match history_table.filter(user_id_column.eq(user_id)).load(conn) {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("failed to retrieve records: {}", err);
                Err(err)
            }
        }
    }
}
