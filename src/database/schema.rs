// @generated automatically by Diesel CLI.

diesel::table! {
    wallets (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
        volume -> Int4,
        enterprise_id -> Int4,
    }
}
