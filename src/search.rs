use std::ops::Range;

use query::Query;

pub struct SearchRequest {
    pub query: Query,
    pub start: usize,
}

pub struct SearchResponse {
    pub query: Query,
    pub match_info: MatchInfo,
}

impl SearchResponse {
    pub fn new(query: Query, line_indices: Vec<usize>, range: Range<usize>) -> Self {
        SearchResponse {
            query: query,
            match_info: MatchInfo {
                line_indices: line_indices,
                range: range,
            }
        }
    }
}

pub struct MatchInfo {
    pub line_indices: Vec<usize>,
    pub range: Range<usize>,
}
