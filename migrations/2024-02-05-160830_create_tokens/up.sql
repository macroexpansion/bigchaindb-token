-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tokens (
  id serial primary key,
  wallet_id int not null references wallets (id),
  token text not null,
  volume int not null
)
