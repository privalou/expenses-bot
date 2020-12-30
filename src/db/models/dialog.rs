use std::result;

use crate::db::schema::dialogs;
use diesel::prelude::*;
use diesel::result::Error;
use log::{error, info};

use crate::db::schema::dialogs::columns::{
    command as command_column, step as step_column, user_id as user_id_column,
};

use crate::db::Connection;

use crate::db::dialogs as dialogs_table;

type Result<T> = result::Result<T, Error>;

#[derive(Debug, Queryable, AsChangeset, PartialEq, Insertable)]
#[table_name = "dialogs"]
pub struct DialogEntity {
    pub user_id: String,
    pub command: String,
    pub step: Option<String>,
}

impl DialogEntity {
    pub fn new(user_id: String, command: String, step: Option<String>) -> Self {
        DialogEntity {
            user_id,
            command,
            step,
        }
    }

    pub fn save_dialog(dialog: &DialogEntity, conn: &Connection) -> Result<()> {
        info!("insert or update of dialog {:?}", dialog);
        match diesel::insert_into(dialogs_table)
            .values(dialog)
            .on_conflict(user_id_column)
            .do_update()
            .set(dialog)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("failed to insert or update dialog: {}", err);
                Err(err)
            }
        }
    }

    pub fn get_user_dialog(user_id: &str, conn: &Connection) -> Result<DialogEntity> {
        info!("get dialog for user: {} ", user_id);
        match dialogs_table
            .filter(user_id_column.eq(user_id))
            .first::<DialogEntity>(conn)
        {
            Ok(dialog) => Ok(dialog),
            Err(err) => {
                error!("failed to retrieve dialog: {}", err);
                Err(err)
            }
        }
    }

    pub fn update_dialog(dialog_entity: &DialogEntity, conn: &Connection) -> Result<()> {
        info!("update dialog dialog entity: {:?}", dialog_entity);
        let updated_row =
            diesel::update(dialogs_table.filter(user_id_column.eq(&dialog_entity.user_id)))
                .set((
                    command_column.eq(&dialog_entity.command),
                    step_column.eq(&dialog_entity.step),
                ))
                .get_result::<DialogEntity>(conn);
        match updated_row {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(
                    "Can not update dialog for user: {}, {}",
                    dialog_entity.user_id, err
                );
                Err(err)
            }
        }
    }
}
