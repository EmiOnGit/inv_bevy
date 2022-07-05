mod itemslot;
pub mod item_map;
pub mod plugin;
pub mod capacity;

use self::itemslot::ItemSlot;
use bevy::prelude::*;
use capacity::{CapacitySlotHandler, StackHandler};
use item_map::ItemMap;
use ron::de::from_reader;
use std::{fs::File, marker::PhantomData};

pub trait InventoryItem {
    fn name(&self) -> &'static str;
    fn max_stack(&self) -> u32;
}
pub struct InventoryBuilder <Item> {
    _id: &'static str,
    data: PhantomData<Item>,
    item_file: Option<&'static str>,
    inventory_file: Option<&'static str>,
    slot_capacity_validater: Option<Box<dyn CapacitySlotHandler<Item>>>

}
impl <Item> InventoryBuilder <Item>{
    pub fn new(id: &'static str) -> Self {
        InventoryBuilder {
            _id: id,
            data: PhantomData,
            item_file: None,
            inventory_file: None,
            slot_capacity_validater: None,
        }
    }
    pub fn with_items(mut self, path: &'static str) -> Self {
        self.item_file = Some(path);
        self
    }
    pub fn with_inventory_file(mut self, path: &'static str) -> Self {
        self.inventory_file = Some(path);
        self
    }
    pub fn with_slot_capacity_validater(mut self, slot_capacity_validater: Box<dyn CapacitySlotHandler<Item>>) -> Self{
        self.slot_capacity_validater = Some(slot_capacity_validater);
        self
    }
}

impl <Item> InventoryBuilder <Item>{
    pub fn build(self) -> Inventory<Item> {
        let mut item_slots = vec![None;10];
        
        if let Some(inv_file) = self.inventory_file {
            let f = File::open(inv_file).expect("Failed opening file");
            let inv: Vec<(String, usize)>  = match from_reader(f) {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to load config: {}", e);
                    std::process::exit(1);
                }
            };
            for (index,item) in inv.into_iter().enumerate() {
                item_slots[index] = Some(ItemSlot {
                    item: item.0,
                    count: item.1
                })
            }
        }
        Inventory {
            item_slots,
            active: 0,
            slot_capacity_validater: Box::new(StackHandler),
            _capacity_handler: CapacityHandler::None,
        }
    }
}

#[derive(Component)]
pub struct Inventory<Item> {
    item_slots: Vec<Option<ItemSlot>>,
    active: usize,
    _capacity_handler: CapacityHandler,
    slot_capacity_validater: Box<dyn CapacitySlotHandler<Item>>

}

impl <Item> Inventory<Item>{
    pub fn print(&self) {
        for item in &self.item_slots {
            println!("{:?}", item);
        }
    }
    pub fn get_items(&self) -> Vec<&String> {
        self.item_slots.iter()
        .filter(|slot| slot.is_some())
        .map(|slot| &slot.as_ref().unwrap().item)
        .collect()
    }
    pub fn get(&self) -> Option<&str> {
        match &self.item_slots[self.active] {
            Some(slots) => Some(slots.item_name()),
            None => None
        }
    }
    pub fn add_item(&mut self, item_map: &ItemMap<Item>, item_id: &str) -> bool {
        // checks if the item can be added to an existing slot
        for slot in &mut self.item_slots {
            if let Some(slot) = slot {
                if slot.item.as_str() == item_id {
                    if self.slot_capacity_validater.can_add(item_map, &slot) {
                        slot.count += 1;
                        return true;
                    }
                }
            }
        }
        // checks if the item can init a new slot
        for index in 0..self.item_slots.len() {
            if self.item_slots[index].is_none() {
                self.item_slots[index] = Some(ItemSlot {
                    item: item_id.to_string(),
                    count: 1,
                });
                return true;
            }
        }

        false
    }
    pub fn get_position(&self, index: usize) -> Option<&str> {
        match &self.item_slots[index] {
            Some(slots) => Some(slots.item_name()),
            None => None
        }
    }
}

pub enum CapacityHandler {
    None,
    Weight(f32),
    Count(u32),
}