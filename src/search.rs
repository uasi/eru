use query::Query;

pub struct SearchRequest {
    pub query: Query,
    pub start: usize,
}
