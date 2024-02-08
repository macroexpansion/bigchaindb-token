-- Your SQL goes here
CREATE TABLE IF NOT EXISTS wallets_tokens (
  wallet_id int not null references wallets (id),
  token_id int not null references tokens (id),
  volume int not null,
  primary key (wallet_id, token_id)
)
