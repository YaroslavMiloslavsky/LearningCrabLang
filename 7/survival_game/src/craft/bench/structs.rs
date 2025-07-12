use crate::{
    craft::recipe::structs::CraftingRecipe,
    inventory::{items::structs::Item, management::structs::Inventory},
};

pub struct CraftingBench {
    recipes: Vec<CraftingRecipe>,
}

impl CraftingBench {
    pub fn new(recipes: Vec<CraftingRecipe>) -> Self {
        Self { recipes }
    }
}

impl CraftingBench {
    pub fn craft(&self, inventory: &mut Inventory, recipe_name: &str) -> Result<Item, String> {
        if let Some(recipe) = self.recipes.iter().find(|r| r.name.eq(recipe_name)) {
            for (needed_item, needed_quant) in &recipe.inputs {
                let inventory_has = inventory.count_item(&needed_item.name);
                if inventory_has != *needed_quant {
                    return Err(format!(
                        "Needed {needed_quant} {:#?}, got {inventory_has} {:#?}",
                        needed_item.category, needed_item.category
                    ));
                };
            }

            // This means that the inventory has everything it needs for the recipe
            let output = &recipe.output; // Without lifetimes I see no way around it, by your design add_item receives am item an not a ref!!
            match inventory.add_item(output.clone()) {
                Ok(()) => {
                    for (needed_item, needed_quant) in &recipe.inputs {
                        for _ in 0..*needed_quant {
                            inventory.remove_item(&needed_item.name);
                        }
                    }
                    Ok(output.clone())
                }
                Err(e) => Err(format! {"Inventory full, cannot add crafted item: {e}"}),
            }
        } else {
            Err(String::from("No recipe called {recipe_name}"))
        }
    }
}
