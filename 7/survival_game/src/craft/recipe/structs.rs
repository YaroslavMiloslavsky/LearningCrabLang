use std::collections::HashMap;

use crate::inventory::items::structs::Item;

pub struct CraftingRecipe {
    pub name: String,
    pub output: Item,
    pub inputs: HashMap<Item, u32>,
}

impl CraftingRecipe {
    pub fn new(name: String, output: Item, inputs: HashMap<Item, u32>) -> Self {
        Self {
            name,
            output,
            inputs,
        }
    }
}
