mod migration;
pub mod pagination;
pub mod schema;

use diesel::{pg::PgConnection, Connection};
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub type DbPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct DatabaseConnPool;

impl DatabaseConnPool {
    pub async fn new(db_url: &str) -> DbPool {
        // run migration
        let mut conn = PgConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
        migration::run_migrations(&mut conn).unwrap();

        // create connection pool
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = bb8::Pool::builder()
            .build(config)
            .await
            .expect("unable to create database connection pool");
        pool
    }
}
