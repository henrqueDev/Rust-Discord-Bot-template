// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        id_user_discord -> Int8,
        id_server_discord -> Int8,
    }
}
