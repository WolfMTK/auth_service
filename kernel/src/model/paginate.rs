pub struct LimitAndQuery {
    pub limit: i32,
    pub offset: i32,
}

impl LimitAndQuery {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self { limit, offset }
    }
}
