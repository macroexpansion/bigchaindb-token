use std::sync::Arc;

use crate::{config::Config, database::DbPool};

pub struct Repo {}

impl Repo {
    pub fn new(db_pool: DbPool) -> Self {
        todo!()
    }
}

pub struct Service {
    pub config: Arc<Config>,
    pub repo: Arc<Repo>,
}

impl Service {
    pub fn new(config: Arc<Config>, repo: Arc<Repo>) -> Self {
        todo!()
    }
}
