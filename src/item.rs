use std::sync::Arc;

pub struct Item {
    pub string: Arc<String>,
}

impl Item {
    pub fn new(string: Arc<String>) -> Self {
        Item {
            string: string,
        }
    }
}
