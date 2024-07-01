use serde::Deserialize;
use validator::Validate;

use application::model::paginate::PaginateView;

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginateQuery {
    #[serde(default = "default_limit")]
    #[validate(range(min = 0, message = "`limit` incorrect value."))]
    pub limit: u32,
    #[serde(default = "default_offset")]
    #[validate(range(min = 0, message = "`offset` incorrect value"))]
    pub offset: u32,
}

fn default_limit() -> u32 {
    10
}

fn default_offset() -> u32 {
    0
}

impl From<PaginateQuery> for PaginateView {
    fn from(value: PaginateQuery) -> Self {
        Self {
            limit: value.limit,
            offset: value.offset,
        }
    }
}
