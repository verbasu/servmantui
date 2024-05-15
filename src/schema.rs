// @generated automatically by Diesel CLI.

diesel::table! {
    services (id) {
        id -> Int4,
        name -> Varchar,
        back_path -> Text,
        front_path -> Text,
        back_env -> Nullable<Text>,
        front_env -> Nullable<Text>,
        icon_url -> Nullable<Text>,
        active -> Bool,
    }
}
