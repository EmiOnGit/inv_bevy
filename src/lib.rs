mod itemslot;

use self::itemslot::ItemSlot;
use bevy::prelude::*;
use std::fmt::Debug;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Inventory<ItemType> {
    item_slots: Vec<Option<ItemSlot<ItemType>>>,
    active: usize,
    _capacity_handler: CapacityHandler,
}

impl<ItemType: PartialEq + Clone> Inventory<ItemType> {
    pub fn add_item(&mut self, item: ItemType) {
        for slot in self.item_slots.iter_mut() {
            match slot {
                Some(slot) => {
                    if slot.max_stack > slot.count {
                        slot.add();
                        return;
                    }
                }
                None => {}
            }
        }
        let i = self.item_slots.iter().position(|x| x.is_none());
        if let Some(i) = i {
            self.item_slots[i] = Some(ItemSlot {
                name: "apple",
                count: 1,
                item,
                max_stack: 5,
                icon: None
            });
        }
    }
    pub fn use_item(&mut self) -> Option<ItemType> {
        let slot = &mut self.item_slots[self.active];

        match slot {
            Some(slot) => {
                let item = slot.get();
                if slot.is_empty() {
                    self.item_slots[self.active] = None;
                }
                Some(item)
            }
            None => None,
        }
    }
}

impl<ItemType> Inventory<ItemType>
where
    ItemType: Debug,
{
    pub fn print(&self) {
        for item in &self.item_slots {
            println!("{:?}", item);
        }
    }
}
impl<ItemType: Clone> Default for Inventory<ItemType> {
    fn default() -> Self {
        Inventory {
            item_slots: vec![None; 10],
            active: 0,
            _capacity_handler: CapacityHandler::Count(10),
        }
    }
}
pub enum CapacityHandler {
    _None,
    _Weight(f32),
    Count(u32),
}
