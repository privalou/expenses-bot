table! {
    dialogs (user_id) {
        user_id -> Text,
        command -> Text,
        step -> Nullable<Text>,
    }
}

table! {
    history (id) {
        id -> Integer,
        user_id -> Text,
        amount -> Float,
        category -> Nullable<Text>,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Text,
        currency -> Nullable<Text>,
    }
}

joinable!(dialogs -> users (user_id));
joinable!(history -> users (user_id));

allow_tables_to_appear_in_same_query!(dialogs, users, history);
