use std::sync::Arc;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    config::Config,
    database::{schema::edges_wallets, DbPool},
};

use super::model;

pub struct EdgeService {
    config: Arc<Config>,
    pool: DbPool,
}

impl EdgeService {
    pub fn new(config: Arc<Config>, pool: DbPool) -> Self {
        tracing::info!("initialized");

        Self { config, pool }
    }

    pub async fn get_edge_to_wallet(&self, edge_id: i32) -> anyhow::Result<model::EdgeWallet> {
        use crate::database::schema::edges_wallets::dsl;

        let mut conn = self.pool.get().await.unwrap();

        let edge = dsl::edges_wallets
            .filter(model::EdgeWallet::with_edge_id(edge_id))
            .select(model::EdgeWallet::as_select())
            .first(&mut conn)
            .await?;

        Ok(edge)
    }

    pub async fn create_edge_to_wallet(
        &self,
        edge_id: i32,
        src_wallet_id: i32,
        dst_wallet_id: i32,
    ) -> anyhow::Result<model::EdgeWallet> {
        let mut conn = self.pool.get().await.unwrap();
        let edge = diesel::insert_into(edges_wallets::table)
            .values(model::NewEdgeWallet {
                edge_id,
                src_wallet_id,
                dst_wallet_id,
            })
            .returning(model::EdgeWallet::as_returning())
            .get_result(&mut conn)
            .await?;
        Ok(edge)
    }
}
