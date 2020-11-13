use std::collections::BTreeMap;

use crate::search::MatchInfo;

pub struct MatchInfoCache {
    cache: BTreeMap<String, MatchInfo>,
}

impl MatchInfoCache {
    pub fn new() -> Self {
        MatchInfoCache {
            cache: BTreeMap::new(),
        }
    }

    pub fn get(&self, query_string: &str) -> Option<&MatchInfo> {
        self.cache.get(query_string)
    }

    pub fn insert(&mut self, query_string: String, info: MatchInfo) {
        self.cache
            .entry(query_string)
            .or_insert_with(Default::default)
            .merge(info);
    }
}
