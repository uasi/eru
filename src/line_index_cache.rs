use std::collections::BTreeMap;
use std::ops::Range;

pub struct LineIndexCache {
    cache: BTreeMap<String, (Vec<usize>, usize)>,
}

impl LineIndexCache {
    pub fn new() -> Self {
        LineIndexCache {
            cache: BTreeMap::new(),
        }
    }

    pub fn get(&self, query_string: &String) -> Option<&(Vec<usize>, usize)> {
        self.cache.get(query_string)
    }

    pub fn put(&mut self, query_string: String, line_indices: Vec<usize>, range: Range<usize>) {
        if let Some((mut indices, end)) = self.cache.remove(&query_string) {
            assert!(range.start <= end,
                "range mismatch: query={:?} indices={:?} range={:?}, expected range.start<={:?}", query_string, line_indices, range, end);
            let overlap_len = end - range.start;
            indices.extend(line_indices.into_iter().skip(overlap_len));
            self.cache.insert(query_string, (indices, range.end));
            return;
        }
        self.cache.insert(query_string, (line_indices, range.end));
    }
}
