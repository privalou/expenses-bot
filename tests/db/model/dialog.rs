use diesel::result::Error;
use diesel::Connection;

use crate::db::model::test_helper::establish_connection;
use bot::bot::dialogs::Command;
use bot::db::models::dialog::DialogEntity;
use bot::db::models::user::UserEntity;

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
            DialogEntity::new(
                USER_ID.to_string(),
                Command::Start.to_string(),
                Some("CurrencySelection".to_string()),
            ),
            dialog_option
        );
        Ok(())
    });
}
