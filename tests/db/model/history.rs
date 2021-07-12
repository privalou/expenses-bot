use diesel::result::Error;
use diesel::Connection;

use crate::db::model::test_helper::establish_connection;
use bot::db::models::history::HistoryRepository;
use bot::db::models::user::UserEntity;

#[test]
fn users_integration_test() {
    let conn = establish_connection();
    conn.test_transaction::<_, Error, _>(|| {
        UserEntity::save_user("user_id", &conn).unwrap();
        HistoryRepository::add_expense_record("user_id".to_string(), 123.00, &conn).unwrap();
        Ok(())
    });
}
