// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Integer,
        ip_address -> Text,
        // Comment Information
        username -> Text,
        comment -> Text,
        timestamp -> Text,
        visible -> Bool,
    }
}