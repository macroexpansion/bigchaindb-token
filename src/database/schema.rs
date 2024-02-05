// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Int4,
        wallet_id -> Int4,
        token -> Text,
        volume -> Int4,
    }
}

diesel::table! {
    wallets (id) {
        id -> Int4,
        public_key -> Text,
        private_key -> Text,
    }
}

diesel::joinable!(tokens -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    wallets,
);
