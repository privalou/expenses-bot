use std::env;

use expenses::db::{Connection, DbConnectionPool};

pub fn establish_connection() -> Connection {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let db_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL environment variable or configure it at test.env file");
    let pool = DbConnectionPool::new(&db_url);
    pool.establish_connection()
}
