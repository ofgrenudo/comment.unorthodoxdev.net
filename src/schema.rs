// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Nullable<Integer>,
        ip_address -> Text,
        related_post_id -> Nullable<Text>,
        username -> Text,
        comment -> Text,
        time_stamp -> Text,
        visible -> Integer,
    }
}
