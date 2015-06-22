use std::ops::Range;

use query::Query;

pub struct SearchRequest {
    pub query: Query,
    pub start: usize,
}

pub struct SearchResponse {
    pub query: Query,
    pub match_info: (Vec<usize>, Range<usize>),
}
