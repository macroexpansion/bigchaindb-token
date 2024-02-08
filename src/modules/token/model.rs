use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::schema::tokens;

#[derive(Debug, Serialize, Selectable, Queryable, ToSchema)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    #[serde(skip_serializing)]
    pub id: i32,
    pub token: String,
}

type WithToken<'a> = diesel::dsl::Eq<tokens::token, &'a str>;

impl Token {
    pub fn with_token(token: &str) -> WithToken {
        tokens::token.eq(token)
    }
}

#[derive(Deserialize, Insertable, ToSchema)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewToken {
    pub token: String,
}
