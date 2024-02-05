-- Your SQL goes here
CREATE TABLE IF NOT EXISTS wallets (
  id serial primary key,
  public_key text not null,
  private_key text not null
)
