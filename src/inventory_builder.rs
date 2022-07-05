use std::{marker::PhantomData, fs::File};

use ron::de::from_reader;

use crate::{capacity::{CapacitySlotHandler, StackHandler}, Inventory, itemslot::ItemSlot, CapacityHandler};

pub struct InventoryBuilder <Item> {
    _id: &'static str,
    data: PhantomData<Item>,
    inventory_file: Option<&'static str>,
    slot_capacity_validater: Option<Box<dyn CapacitySlotHandler<Item>>>
}

impl <Item> InventoryBuilder <Item>{
    pub fn new(id: &'static str) -> Self {
        InventoryBuilder {
            _id: id,
            data: PhantomData,
            inventory_file: None,
            slot_capacity_validater: None,
        }
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
            // String::new("hi").
            let f = File::open(inv_file).expect(format!("Failed finding inventory file for {}", self._id).as_str());
            let inv: Vec<(String, usize)>  = match from_reader(f) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("Failed to parse inventory file for {}\n with error {}", self._id, e);
                    eprintln!("init as empty inventory");
                    vec![]
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