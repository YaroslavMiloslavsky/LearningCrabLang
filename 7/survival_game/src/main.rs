#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod craft;
mod inventory;

use std::io;

use inventory::items::enums::ConsumableType;
use inventory::items::enums::ItemType;
use inventory::items::enums::WeaponType;
use inventory::items::structs::Item;
use inventory::management::structs::Inventory;

fn main() {
    println!("------------Welcome to the crafting game!-----------");
    println!("First let's create an inventory:");
    println!("50 wight units should be enough for you");
    let mut inventory = Inventory::new(50);
    let mut input = String::new();

    loop {
        input.clear();
        println!("Pick an action:");
        println!("1 - Add an item");
        println!("2 - Craft a recipe");
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read the input!!");
        input = input.trim().to_lowercase();
        if input == "1" {
            add_item();
        }
        else if input == "2" {
            craft_recipe();
        }
    }
}

fn add_item() {
    // TODO: Add item dynamically
}

fn craft_recipe() {

}
