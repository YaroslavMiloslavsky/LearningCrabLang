#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod craft;
mod inventory;

use std::collections::HashMap;

use inventory::items::enums::ItemType;
use inventory::items::enums::ToolType;
use inventory::items::structs::Item;
use inventory::management::structs::Inventory;

use crate::craft::bench::structs::CraftingBench;
use crate::craft::recipe::structs::CraftingRecipe;
use crate::inventory::items::enums::MaterialType;

const WEIGHT_UNITS: u32 = 10;

fn main() {
    println!("------------Welcome to the crafting game!-----------");
    println!("First let's create an inventory:");
    println!("{WEIGHT_UNITS} wight units should be enough for you");
    let mut inventory = Inventory::new(WEIGHT_UNITS);

    let wood = Item::new(String::from("wood"), ItemType::Material(MaterialType::Wood));
    let axe = Item::new(String::from("axe"), ItemType::Tool(ToolType::Axe));
    let stick = Item::new(
        String::from("stick"),
        ItemType::Material(MaterialType::Stick),
    );

    let mut wooden_axe_inputs: HashMap<Item, u32> = HashMap::with_capacity(2);
    wooden_axe_inputs.insert(wood.clone(), 2);
    wooden_axe_inputs.insert(stick.clone(), 1);

    checked_add_items(&mut inventory, vec![wood.clone(), wood, axe, stick]);

    let wooden_axe_recipe = CraftingRecipe::new(
        String::from("wooden axe"),
        Item::new(String::from("wooden axe"), ItemType::Tool(ToolType::Axe)),
        wooden_axe_inputs,
    );
    let work_bench = CraftingBench::new(vec![wooden_axe_recipe]);
    match work_bench.craft(&mut inventory, "wooden axe") {
        Ok(val) => {
            println!("Created {val:#?}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
    inventory.list_items();
}

fn checked_add_item(inventory: &mut Inventory, item: Item) {
    match inventory.add_item(item) {
        Ok(()) => {
            inventory.list_items();
        }
        Err(e) => println!("Could not add to inventory :{e}"),
    }
}

fn checked_add_items(inventory: &mut Inventory, items: Vec<Item>) {
    for item in items {
        match inventory.add_item(item) {
            Ok(()) => {
                inventory.list_items();
            }
            Err(e) => println!("Could not add to inventory :{e}"),
        }
    }
}

/*
    I did all of :
    Initialize an Inventory with a reasonable max_capacity.

Create several Item instances (materials, tools, etc.).

Add some materials to the Inventory using add_item().

Define a few CraftingRecipe instances (e.g., "Wooden Axe" from Log and Stick; "Cooked Meat" from RawMeat).

Initialize a CraftingBench with your recipes.

Attempt to craft a "Wooden Axe" (you should have enough materials).

Attempt to craft something you don't have materials for (to test Err path).

Attempt to craft something when the inventory is almost full, so add_item fails due to capacity (to test that Err path).

Use list_items() and current_weight() to show the state of the inventory before and after crafting attempts.

And it seemed to work
*/


/*
    Conceptual Questions 
    Collection Choices 
    Vec is best suited for many inserts without many reads.
    Map is better for many reads but is more of work to insert.
    String is a struct which is saved on the heap, while &str is saved on the stack.
    But to user &str in my coe I would need lifetimes.

    For the last question, see my impl.

    Ownership & Borrowing with Collections
    This caused me some pain here, I will not lie. But there are no dangling pointes and C++ common pitfalls in this code. This code uses clone too much, but once we cover lifeteimes I will be able to construct structs with refs instead of heap allocations all the time
    in the craft method a the pitfall was returning the output due to borrow checked fighting with me
*/