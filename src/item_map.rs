use std::collections::HashMap;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct ItemMap<ItemLayout> {
    items: HashMap<String, ItemLayout>
}

impl <ItemLayout> ItemMap<ItemLayout>{
    pub fn get(&self, key: String) -> Option<&ItemLayout> {
        self.items.get(&key)
    }
}