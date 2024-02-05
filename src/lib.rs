pub mod config;
pub mod constant;
pub mod database;
pub mod doc;
pub mod error;
pub mod middleware;
pub mod modules;
pub mod request;
pub mod response;
pub mod state;

pub mod fallback {
    use axum::{http::StatusCode, response::IntoResponse};

    pub async fn handler_404() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, "nothing to see here")
    }
}

pub mod extract {
    use axum::{
        async_trait,
        extract::{FromRef, FromRequestParts},
        http::{request::Parts, StatusCode},
    };
    use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

    use crate::{database::DbPool, error::internal_error};

    pub struct DatabaseConnection(
        pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
    );

    #[async_trait]
    impl<S> FromRequestParts<S> for DatabaseConnection
    where
        S: Send + Sync,
        DbPool: FromRef<S>,
    {
        type Rejection = (StatusCode, String);

        async fn from_request_parts(
            _parts: &mut Parts,
            state: &S,
        ) -> Result<Self, Self::Rejection> {
            let pool = DbPool::from_ref(state);

            let conn = pool.get_owned().await.map_err(internal_error)?;

            Ok(Self(conn))
        }
    }
}
