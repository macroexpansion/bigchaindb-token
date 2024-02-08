pub mod edge;
pub mod token;
pub mod wallet;
pub mod wallet_to_token;

use std::sync::Arc;

use crate::{
    config::Config,
    database::DbPool,
    modules::{
        edge::EdgeService, token::TokenService, wallet::WalletService,
        wallet_to_token::WalletTokenService,
    },
};

pub struct Repo {}

impl Repo {
    pub fn new(db_pool: DbPool) -> Self {
        todo!()
    }
}

pub struct Service {
    pub config: Arc<Config>,
    pub wallet: Arc<WalletService>,
    pub token: Arc<TokenService>,
    pub wallet_to_token: Arc<WalletTokenService>,
    pub edge: Arc<EdgeService>,
}

impl Service {
    pub fn new(config: Arc<Config>, pool: DbPool) -> Self {
        let token = Arc::new(TokenService::new(config.clone(), pool.clone()));
        let edge = Arc::new(EdgeService::new(config.clone(), pool.clone()));
        let wallet_to_token = Arc::new(WalletTokenService::new(config.clone(), pool.clone()));
        Self {
            config: config.clone(),
            token: token.clone(),
            edge: edge.clone(),
            wallet_to_token: wallet_to_token.clone(),
            wallet: Arc::new(WalletService::new(
                config.clone(),
                pool.clone(),
                token.clone(),
                wallet_to_token.clone(),
                edge.clone(),
            )),
        }
    }
}
