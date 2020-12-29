use std::env;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{Connection, PgConnection};

use expenses::db::migrate_and_config_db;
use expenses::db::models::user::UserEntity;

#[test]
fn users_integration_test() {
    let conn = establish_connection();
    conn.test_transaction::<_, Error, _>(|| {
        let users = UserEntity::get_users(&conn).unwrap();
        assert!(users.is_empty());
        UserEntity::save_user("user_id", &conn).unwrap();
        UserEntity::save_user("user_id1", &conn).unwrap();
        let users = UserEntity::get_users(&conn).unwrap();
        assert_eq!(2, users.len());
        assert!(UserEntity::is_registered("user_id", &conn).unwrap());
        assert!(!UserEntity::is_registered("not_existing_user", &conn).unwrap());
        let updated_users = UserEntity::update_currency("$", "user_id", &conn).unwrap();
        assert_eq!(1, updated_users);
        let updated_users = UserEntity::update_currency("$", "not_existing_user", &conn).unwrap();
        assert_eq!(0, updated_users);
        Ok(())
    });
}

fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    dotenv::from_filename("test.env").expect("Failed to read env variables from test.env");
    let db_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL environment variable or configure it at test.env file");
    let pool = migrate_and_config_db(&db_url);
    pool.get().unwrap()
}
