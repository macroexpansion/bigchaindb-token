use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, Copy, Clone, IntoParams)]
pub struct Pagination {
    #[param(default = 1)]
    pub page_num: i64,
    #[param(default = 10)]
    pub page_size: i64,
}
