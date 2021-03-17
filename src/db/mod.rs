use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool as R2D2Pool, PooledConnection},
    RunQueryDsl
};

use log::info;


use crate::db::schema::{
    dialogs::dsl::dialogs,
    history::dsl::history,
    users::dsl::users
};

mod schema;

pub mod models;

embed_migrations!();

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;


pub struct DbConnectionPool {
    pool: Pool,
}

impl DbConnectionPool {
    pub fn new(url: &str) -> Self {
        info!("Migrating and configurating database...");
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = R2D2Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        embedded_migrations::run(&pool.get().expect("Failed to get connection."))
            .expect("Failed to run migrations");
        DbConnectionPool { pool }
    }

    pub fn establish_connection(&self) -> Connection {
        self.pool.get().expect("Can not get connection from pool")
    }
}

pub fn clear_tables(conn: &Connection) -> usize {
    let dialogs_deleted = diesel::delete(dialogs).execute(conn);
    let history_records_deleted = diesel::delete(history).execute(conn);
    let users_deleted = diesel::delete(users).execute(conn);
    dialogs_deleted.unwrap_or(0) + users_deleted.unwrap_or(0) + history_records_deleted.unwrap_or(0)
}
