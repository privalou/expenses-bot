use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection, RunQueryDsl};
use log::info;

use crate::db::schema::dialogs::dsl::dialogs;
use crate::db::schema::history::dsl::history;
use crate::db::schema::users::dsl::users;

mod schema;

pub mod models;

embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_config_db(url: &str) -> Pool {
    info!("Migrating and configurating database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to get connection."))
        .expect("Failed to run migrations");

    pool
}

pub fn clear_tables(conn: &Connection) -> usize {
    let dialogs_deleted = diesel::delete(dialogs).execute(conn);
    let history_records_deleted = diesel::delete(history).execute(conn);
    let users_deleted = diesel::delete(users).execute(conn);
    dialogs_deleted.unwrap_or(0) + users_deleted.unwrap_or(0) + history_records_deleted.unwrap_or(0)
}
