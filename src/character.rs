use std::collections::HashMap;
use crate::item::Item;

pub trait Character {
    fn set_items(&mut self, new_items: HashMap<Item, i32>);
    fn add_item(&mut self, item: Item, amount: i32);
    fn remove_item(&mut self, item: Item, amount: i32) -> bool;
    fn get_items(&self) -> &HashMap<Item, i32>;
    fn print_self(&self);
}