mod itemslot;
pub mod item_map;

use self::itemslot::ItemSlot;
use bevy::prelude::*;
use std::fs::File;
use ron::de::from_reader;

pub trait InventoryItem {
    fn name(&self) -> &'static str;
    fn max_stack(&self) -> u32;
}
pub struct InventoryBuilder {
    id: &'static str,
    item_file: Option<&'static str>,
    inventory_file: Option<&'static str>,
}
impl InventoryBuilder {
    pub fn new(id: &'static str) -> Self {
        InventoryBuilder {
            id,
            item_file: None,
            inventory_file: None,
        }
    }
    pub fn with_items(mut self, path: &'static str) -> Self{
        self.item_file = Some(path);
        self
    }
    pub fn with_inventory_file(mut self, path: &'static str) -> Self {
        self.inventory_file = Some(path);
        self
    }
}

impl InventoryBuilder{
    pub fn build(self) -> Inventory {
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
            _capacity_handler: CapacityHandler::None,
        }
    }
}

#[derive(Component)]
pub struct Inventory {
    item_slots: Vec<Option<ItemSlot>>,
    active: usize,
    _capacity_handler: CapacityHandler,
    // item_map: Handle<ItemMap>
}

impl Inventory{
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
}
pub enum CapacityHandler {
    None,
    Weight(f32),
    Count(u32),
}