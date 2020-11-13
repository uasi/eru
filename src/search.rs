use std::ops::Range;

use crate::query::Query;

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
            query,
            match_info: MatchInfo {
                line_indices,
                index_range,
            },
        }
    }
}

#[derive(Debug)]
pub struct MatchInfo {
    pub line_indices: Vec<usize>,
    pub index_range: Range<usize>,
}

impl MatchInfo {
    pub fn merge(&mut self, other: Self) {
        assert!(self.index_range.start <= other.index_range.end);
        let overlap_len = self.index_range.end - other.index_range.start;
        self.line_indices
            .extend(other.line_indices.into_iter().skip(overlap_len));
        if self.index_range.end < other.index_range.end {
            self.index_range.end = other.index_range.end;
        }
    }
}

impl Default for MatchInfo {
    fn default() -> Self {
        MatchInfo {
            line_indices: Vec::new(),
            index_range: 0..0,
        }
    }
}
