use application::model::paginate::PaginateView;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginateQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default = "default_offset")]
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
