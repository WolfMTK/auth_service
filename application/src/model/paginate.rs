use kernel::model::paginate::LimitAndQuery;

pub struct PaginateView {
    pub limit: u32,
    pub offset: u32,
}

impl PaginateView {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset }
    }
}

impl From<PaginateView> for LimitAndQuery {
    fn from(value: PaginateView) -> Self {
        LimitAndQuery::new(value.limit as i32, value.offset as i32)
    }
}
