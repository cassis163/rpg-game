use std::collections::HashMap;
use crate::character::Character;
use crate::item::Item;

pub struct Player {
    pub(crate) name: String,
    pub(crate) items: HashMap<Item, i32>,
}

impl Player {
    pub(crate) fn new(name: String) -> Player {
        Player {
            name,
            items: HashMap::new(),
        }
    }
}

impl Character for Player {
    fn set_items(&mut self, items: HashMap<Item, i32>) {
        self.items = items;
    }

    fn add_item(&mut self, item: Item, amount: i32) {
        for (key, value) in &mut self.items {
            if key.name == item.name {
                *value += amount;
                return;
            }
        }
        self.items.insert(item, amount);
    }


    //noinspection DuplicatedCode
    fn remove_item(&mut self, item: Item, amount: i32) -> bool {
        // If player does not have the item, return false
        if self.items.iter().position(|(key, _value)| key.name == item.name).is_none() {
            return false;
        }
        self.items = self.items.clone().into_iter().filter_map(|(key, value)| {
            if key.name != item.name {
                return Some((key.to_owned(), value));
            }
            if value - amount <= 0 {
                return None;
            }
            Some((key.to_owned(), value - amount))
        }).collect();
        true
    }


    fn get_items(&self) -> &HashMap<Item, i32> {
        &self.items
    }

    fn print_self(&self) {
        println!("{}", self.name);
    }
}