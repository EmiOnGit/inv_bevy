use crate::{item_map::ItemMap, itemslot::ItemSlot};

pub trait CapacitySlotHandler<T>: Sync + Send {
    fn can_add(&self, item_map: &ItemMap<T>, stack: &ItemSlot) -> bool;
}

pub struct StackHandler;
impl <T> CapacitySlotHandler<T> for StackHandler {
    fn can_add(&self, _item_map: &ItemMap<T>, stack: &ItemSlot) -> bool {
        stack.count < 8
    }
}
