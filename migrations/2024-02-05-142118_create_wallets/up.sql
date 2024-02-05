-- Your SQL goes here
CREATE TABLE IF NOT EXISTS wallets (
  id SERIAL PRIMARY KEY,
  user_id int NOT NULL,
  token text UNIQUE NOT NULL,
  volume int UNIQUE NOT NULL,
  enterprise_id int NOT NULL,
  UNIQUE(user_id, enterprise_id)
)
