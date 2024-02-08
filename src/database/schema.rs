// @generated automatically by Diesel CLI.

diesel::table! {
    edges_wallets (id) {
        id -> Int4,
        edge_id -> Int4,
        src_wallet_id -> Int4,
        dst_wallet_id -> Int4,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    wallets (id) {
        id -> Int4,
        public_key -> Text,
        private_key -> Text,
    }
}

diesel::table! {
    wallets_tokens (wallet_id, token_id) {
        wallet_id -> Int4,
        token_id -> Int4,
        volume -> Int4,
    }
}

diesel::joinable!(wallets_tokens -> tokens (token_id));
diesel::joinable!(wallets_tokens -> wallets (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    edges_wallets,
    tokens,
    wallets,
    wallets_tokens,
);
