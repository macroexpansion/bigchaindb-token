use serde::Serialize;
use utoipa::ToSchema;

use crate::database::pagination::LoadCountPages;

#[derive(Serialize, ToSchema)]
pub struct Paginated<T> {
    pub records: Vec<T>,
    pub total_pages: i64,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

impl<T> Paginated<T> {
    pub fn page_num(self, value: i64) -> Self {
        Self {
            records: self.records,
            total_pages: self.total_pages,
            page_num: Some(value),
            page_size: self.page_size,
        }
    }

    pub fn page_size(self, value: i64) -> Self {
        Self {
            records: self.records,
            total_pages: self.total_pages,
            page_num: self.page_num,
            page_size: Some(value),
        }
    }
}

impl<T: Serialize> From<LoadCountPages<T>> for Paginated<T> {
    fn from(value: LoadCountPages<T>) -> Self {
        Self {
            records: value.records,
            total_pages: value.total_pages,
            page_num: None,
            page_size: None,
        }
    }
}
