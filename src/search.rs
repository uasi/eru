use std::ops::Range;

use query::Query;

pub struct Request {
    pub query: Query,
    pub start: usize,
}

pub struct Response {
    pub query: Query,
    pub match_info: MatchInfo,
}

impl Response {
    pub fn new(query: Query, line_indices: Vec<usize>, index_range: Range<usize>) -> Self {
        Response {
            query: query,
            match_info: MatchInfo {
                line_indices: line_indices,
                index_range: index_range,
            }
        }
    }
}

#[derive(Debug)]
pub struct MatchInfo {
    pub line_indices: Vec<usize>,
    pub index_range: Range<usize>,
}
