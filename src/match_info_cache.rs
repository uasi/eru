use std::collections::BTreeMap;

use search::MatchInfo;

pub struct MatchInfoCache {
    cache: BTreeMap<String, MatchInfo>,
}

impl MatchInfoCache {
    pub fn new() -> Self {
        MatchInfoCache {
            cache: BTreeMap::new(),
        }
    }

    pub fn get(&self, query_string: &String) -> Option<&MatchInfo> {
        self.cache.get(query_string)
    }

    pub fn insert(&mut self, query_string: String, info: MatchInfo) {
        self.cache.entry(query_string).or_insert(Default::default()).merge(info);
    }
}
