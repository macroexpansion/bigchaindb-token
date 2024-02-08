-- Your SQL goes here
CREATE TABLE IF NOT EXISTS edges_wallets (
  id serial primary key,
  edge_id int not null,
  src_wallet_id int not null,
  dst_wallet_id int not null,
  foreign key (src_wallet_id) references wallets (id),
  foreign key (dst_wallet_id) references wallets (id)
)
