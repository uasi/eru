use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

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
        match self.cache.entry(query_string) {
            Entry::Occupied(entry) => {
                let cached = entry.into_mut();
                assert!(info.index_range.start <= cached.index_range.end);
                let overlap_len = cached.index_range.end - info.index_range.start;
                cached.line_indices.extend(info.line_indices.into_iter().skip(overlap_len));
                cached.index_range.end = info.index_range.end;
            }
            Entry::Vacant(entry) => {
                entry.insert(info);
            }
        }
    }
}
