use std::env;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use expenses::db::migrate_and_config_db;

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let db_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL environment variable or configure it at test.env file");
    let pool = migrate_and_config_db(&db_url);
    pool.get().unwrap()
}
