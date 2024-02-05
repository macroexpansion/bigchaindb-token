use std::sync::Arc;

use crate::{
    config::Config,
    database::DbPool,
    modules::{Repo, Service},
};

pub struct AppState {
    pub config: Arc<Config>,
    pub repo: Arc<Repo>,
    pub service: Arc<Service>,
}

impl AppState {
    pub fn new(config: Config, db_pool: DbPool) -> Self {
        let config = Arc::new(config);
        let repo = Arc::new(Repo::new(db_pool));
        let service = Arc::new(Service::new(config.clone(), repo.clone()));
        Self {
            config,
            repo,
            service,
        }
    }
}
