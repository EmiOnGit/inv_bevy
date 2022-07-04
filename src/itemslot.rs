use bevy::prelude::*;
#[derive(Debug)]
pub struct ItemSlot<ItemType> {
    pub name: &'static str,
    pub icon: Option<Handle<Image>>,
    pub count: usize,
    pub item: ItemType,
    pub max_stack: usize,
}

impl<ItemType: Clone> Clone for ItemSlot<ItemType> {
    fn clone(&self) -> Self {
        ItemSlot {
            name: self.name,
            count: self.count,
            item: self.item.clone(),
            icon: self.icon.clone(),
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
