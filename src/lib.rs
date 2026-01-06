use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod error;
pub use error::PaginationError;

pub type Result<T> = std::result::Result<T, eywa_errors::AppError>;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    pub page: u32,
    pub limit: u32,
    pub offset: u64,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
    pub has_first: bool,
    pub has_last: bool,
}

impl PaginationParams {
    pub fn from_query(page: Option<u32>, limit: Option<u32>, total: u64) -> Self {
        let page = page.unwrap_or(1).max(1);
        let limit = limit.unwrap_or(10).min(100).max(1) as u64;
        let offset = (page - 1) as u64 * limit;
        let total_pages = if total == 0 {
            0
        } else {
            (total as f64 / limit as f64).ceil() as u32
        };

        Self {
            page,
            limit: limit as u32,
            offset,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
            has_first: total_pages > 0,
            has_last: total_pages > 0,
        }
    }

    pub fn next_page_url(&self, base_url: &str) -> String {
        if self.has_next {
            format!("{}?page={}&limit={}", base_url, self.page + 1, self.limit)
        } else {
            String::new()
        }
    }

    pub fn prev_page_url(&self, base_url: &str) -> String {
        if self.has_prev {
            format!("{}?page={}&limit={}", base_url, self.page - 1, self.limit)
        } else {
            String::new()
        }
    }

    pub fn first_page_url(&self, base_url: &str) -> String {
        if self.has_first {
            format!("{}?page=1&limit={}", base_url, self.limit)
        } else {
            String::new()
        }
    }

    pub fn last_page_url(&self, base_url: &str) -> String {
        if self.has_last {
            format!(
                "{}?page={}&limit={}",
                base_url, self.total_pages, self.limit
            )
        } else {
            String::new()
        }
    }

    pub fn current_page_url(&self, base_url: &str) -> String {
        format!("{}?page={}&limit={}", base_url, self.page, self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationParams,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, pagination: PaginationParams) -> Self {
        Self { data, pagination }
    }
}
