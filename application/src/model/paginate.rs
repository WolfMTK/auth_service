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
        let limit = value.limit;
        let offset = value.offset * limit;
        LimitAndQuery::new(limit as i32, offset as i32)
    }
}
