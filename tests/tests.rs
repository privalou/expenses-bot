extern crate diesel_migrations;

mod bot {
    mod api_integration_test;
}

mod db {
    mod model {
        mod dialog;
        mod test_helper;
        mod user;
    }
}
