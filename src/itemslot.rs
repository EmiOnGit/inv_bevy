#[derive(Debug, Clone)]
pub struct ItemSlot {
    pub item: String,
    pub count: usize,
}


impl ItemSlot {
    pub fn add(&mut self) {
        self.count += 1;
    }
}
impl ItemSlot {
    pub fn get(&mut self) -> String {
        self.count -= 1;
        self.item.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
