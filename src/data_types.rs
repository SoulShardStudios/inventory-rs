use crate::slot_management::swap;

pub trait BaseItem {
    fn is_stackable(&self) -> bool;
    fn max_stack_quantity(&self) -> u16;
    fn name(&self) -> &String;
}

pub trait ItemInstance<Item: BaseItem> {
    fn get_quantity(&self) -> u16;
    fn get_item(&self) -> &Item;
    fn new(item: &Item, quantity: u16) -> Self;
}

pub trait Slot<'a, I: BaseItem, II: ItemInstance<I>> {
    fn get_item_instance(&self) -> Option<II>;
    fn set_item_instance(&mut self, item_instance: Option<II>);
    fn transfer(&mut self, item_instance: Option<II>) -> Option<II> {
        let res = swap(self.get_item_instance(), item_instance);
        self.set_item_instance(res.0);
        res.1
    }

    fn set_change_callback(&mut self, callback: Option<fn(Option<II>)>);
}

pub trait Inventory<'a, I: BaseItem, II: ItemInstance<I>, S: Slot<'a, I, II>> {
    fn size(&self) -> usize;
    fn get_slots(&self) -> &[S];
    fn get_slots_mut(&mut self) -> &mut [S];
}
