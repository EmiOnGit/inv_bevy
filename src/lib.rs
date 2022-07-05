mod itemslot;
pub mod item_map;
pub mod plugin;
pub mod capacity;
pub mod inventory_builder;

use self::itemslot::ItemSlot;
use bevy::prelude::*;
use capacity::{CapacitySlotHandler, StackHandler};
use item_map::ItemMap;

pub trait InventoryItem {
    fn name(&self) -> &'static str;
    fn max_stack(&self) -> u32;
}
/// Container for Inventories. 
/// InventoryBuilder should be used as primarily way to build Inventories.
/// The Item type specifies which kind of item the inventory should be able to hold.
/// # Example
/// ```
/// use bevy::prelude::*;
/// use inv_bevy::*;
/// struct Item {
///     name: String,
///     description: String,
///     weight: u32,
/// }
/// #[derive(Component)]
/// struct Player;
/// fn init_player(commands: &mut Commands) {
///     let inventory = InventoryBuilder::<Item>::new("player_inventory")
///         .with_inventory_file("resources/player_inventory.ron")
///         .build();
///     commands.spawn()
///         .insert(Player)
///         .insert(inventory);
/// }
/// 
/// ```
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
    
    pub fn get_items(&self) -> Vec<&str> {
        self.item_slots.iter()
            .filter(|slot| slot.is_some())
            .map(|slot| slot.as_ref().unwrap().item_name())
            .collect()
    }
    /// returns the currently selected item without changing the inventory state
    /// returns None if the currently active inventory index points to an empty slot
    pub fn get(&self) -> Option<&str> {
        match &self.item_slots[self.active] {
            Some(slots) => Some(slots.item_name()),
            None => None
        }
    }
    /// returns the currently selected item and removes it from the inventory.
    /// If multiple items of the same items are stacked only one will be removed
    pub fn pop_active(&mut self) -> Option<String> {
        let quantity = self.get_quantity();
        match quantity {
            0 => None,
            1 => {
                let name = self.item_slots[self.active].as_ref().unwrap().item_name().to_string();
                self.item_slots[self.active] = None;
                Some(name)
            },
            _ => {
                self.item_slots[self.active].as_mut().unwrap().count -= 1;
                Some(self.item_slots[self.active].as_ref().unwrap().item_name().to_string())
            }
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
    pub fn get_from(&self, index: usize) -> Option<&str> {
        match &self.item_slots[index] {
            Some(slots) => Some(slots.item_name()),
            None => None
        }
    }
    pub fn set_index(&mut self, index: usize) {
        self.active = index;
    }
}

impl <Item> Inventory<Item>{
    fn get_quantity(&self) -> usize {
        match &self.item_slots[self.active] {
            None => 0,
            Some(slot) => {
                slot.count
            }
        }
    }
}
pub enum CapacityHandler {
    None,
    Weight(f32),
    Count(u32),
}