// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Integer,
        small_url -> Text,
        long_url -> Text,
        created_at -> Nullable<Text>,
        delete_at -> Nullable<Text>,
    }
}
