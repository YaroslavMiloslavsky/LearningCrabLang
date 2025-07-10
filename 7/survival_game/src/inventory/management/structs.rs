use crate::inventory::items::structs::Item;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
    max_capacity: u32,
    name_index_map: HashMap<String, Vec<usize>>,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            items: Vec::new(),
            max_capacity: capacity,
            name_index_map: HashMap::new(),
        }
    }
}

impl Inventory {
    fn current_weight(&self) -> u32 {
        let mut current_weight: u32 = 0;
        for item in &self.items {
            current_weight += item.weight;
        }

        current_weight
    }

    fn add_name_index_map(&mut self, index: usize, name: String) {
        if let Some(value) = self.name_index_map.get_mut(&name) {
            value.push(index);
        } else {
            self.name_index_map.insert(name, vec![index]);
        }
    }

    fn remove_name_index_map(&mut self, name: &str) -> Option<usize> {
        if let Some(value) = self.name_index_map.get_mut(name) {
            value.pop()
        } else {
            None
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), String> {
        let capacity_left = self.max_capacity - self.current_weight();
        if capacity_left > 0 {
            self.add_name_index_map(self.items.len(), String::from(&item.name));
            self.items.push(item);
            return Ok(());
        }
        Err(String::from(
            "Couldn't add more items, {capacity_left} units of weight left",
        ))
    }

    pub fn remove_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(val) = self.remove_name_index_map(item_name) {
            let removed = self.items.remove(val);
            return Some(removed);
        }
        None
    }

    pub fn count_item(&self, item_name: &str) -> u32 {
        u32::try_from(
            self.items
                .iter()
                .filter(|item| item.name.eq(item_name))
                .count(),
        )
        .unwrap_or(0)
    }

    pub fn list_items(&self) {
        for item in &self.items {
            println!(
                "Item: {} is a {:?} with weight of {}",
                item.name, item.category, item.weight
            );
        }
    }
}
