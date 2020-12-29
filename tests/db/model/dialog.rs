use std::env;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{Connection, PgConnection};

use expenses::bot::dialogs::Command;
use expenses::db::migrate_and_config_db;
use expenses::db::models::dialog::DialogEntity;
use expenses::db::models::user::UserEntity;

const USER_ID: &str = "user_id";

#[test]
fn dialog_integration_test() {
    let conn = establish_connection();
    conn.test_transaction::<_, Error, _>(|| {
        let result = DialogEntity::get_user_dialog(USER_ID, &conn);
        assert!(result.is_err());
        UserEntity::save_user(USER_ID, &conn).unwrap();
        let dialog_option = DialogEntity::get_user_dialog(USER_ID, &conn).unwrap();
        assert_eq!(
            DialogEntity::new(USER_ID.to_string(), Command::Start.to_string(), None,),
            dialog_option
        );
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
