use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use search::MatchInfo;

pub struct LineIndexCache {
    cache: BTreeMap<String, MatchInfo>,
}

impl LineIndexCache {
    pub fn new() -> Self {
        LineIndexCache {
            cache: BTreeMap::new(),
        }
    }

    pub fn get(&self, query_string: &String) -> Option<&MatchInfo> {
        self.cache.get(query_string)
    }

    pub fn put(&mut self, query_string: String, info: MatchInfo) {
        match self.cache.entry(query_string) {
            Entry::Occupied(entry) => {
                let cached = entry.into_mut();
                assert!(info.range.start <= cached.range.end);
                let overlap_len = cached.range.end - info.range.start;
                cached.line_indices.extend(info.line_indices.into_iter().skip(overlap_len));
                cached.range.end = info.range.end;
            }
            Entry::Vacant(entry) => {
                entry.insert(info);
            }
        }
    }
}
