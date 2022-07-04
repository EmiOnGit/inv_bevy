#[derive(Debug)]
pub struct ItemSlot<ItemType> {
    pub count: usize,
    pub item: ItemType,
    pub max_stack: usize,
}
impl<ItemType: Clone> Clone for ItemSlot<ItemType> {
    fn clone(&self) -> Self {
        ItemSlot {
            count: self.count,
            item: self.item.clone(),
            max_stack: self.max_stack,
        }
    }
}
impl<ItemType: Clone + PartialEq> PartialEq<ItemType> for ItemSlot<ItemType> {
    fn eq(&self, r: &ItemType) -> bool {
        self.item == r.clone()
    }
}
impl<ItemType> ItemSlot<ItemType> {
    pub fn add(&mut self) {
        if self.max_stack > self.count {
            self.count += 1;
        }
    }
}
impl<ItemType: Clone> ItemSlot<ItemType> {
    pub fn get(&mut self) -> ItemType {
        self.count -= 1;
        self.item.clone()
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
