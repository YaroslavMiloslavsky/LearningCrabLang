#![warn(clippy::all, clippy::pedantic)]

const MAX_HULL: u32 = 100;
const MAX_SHIELDS: u32 = 50;
const MAX_FUEL: u32 = 200;
const MAX_CARGO_CAPACITY: usize = 5;

#[derive(Debug)]
struct Ship {
    name: String,
    hull_strength: u32,
    shields: u32,
    fuel: u32,
    cargo: Vec<String>,
}

// Associated functions
impl Ship {
    fn new(name: String) -> Self {
        Self {
            name,
            hull_strength: MAX_HULL,
            shields: MAX_SHIELDS,
            fuel: MAX_FUEL,
            cargo: Vec::with_capacity(MAX_CARGO_CAPACITY),
        }
    }
}

// methods
impl Ship {
    fn display_status(&self) {
        println!("Current ship status:");
        println!("Ship {} full status:", self.name);
        println!("Hull strength is at: {}", self.hull_strength);
        println!("Shields are at: {}", self.shields);
        println!("Fuel is at: {}", self.fuel);
        if self.cargo.is_empty() {
            println!("Currently no cargo");
        } else {
            println!("Current cargo is of size: {}", self.cargo.len());
            println!("And includes:");
            for (index, item) in self.cargo.iter().enumerate() {
                println!("Item {index} is {item}");
            }
        }
    }

    fn take_damage(&mut self, amount: u32) -> bool {
        if self.shields > 0 {
            self.shields = self.shields.saturating_sub(amount);
        }
        else {
            self.hull_strength = self.hull_strength.saturating_sub(amount);
        }
        self.hull_strength == 0
    }

    fn refuel(&mut self, amount: u32) {
        let amount_to_refuel = self.fuel.saturating_add(amount);
        if amount_to_refuel > MAX_FUEL {
            self.fuel = MAX_FUEL;
        } else {
            self.fuel = amount_to_refuel;
        }
    }

    fn load_cargo(&mut self, item: String) -> bool {
        if self.cargo.len() < MAX_CARGO_CAPACITY {
            self.cargo.push(item);
            return true;
        }
        false
    }

    fn jettison_all_cargo(mut self) -> Vec<String> {
        let current_load = std::mem::take(&mut self.cargo);
        self.cargo.clear();
        current_load
    }
}

fn main() {
    let mut cresetl = Ship::new(String::from("Crestel"));
    cresetl.display_status();

    println!("\n\nChecking the ship availability:");
    println!("Ships name: {}", cresetl.name);
    println!("{:#?}\n\n", cresetl.cargo);

    println!("Displaying status again");
    cresetl.display_status();
    println!("\n");

    println!("Testing taking damage:");
    cresetl.display_status();
    println!();
    let mut is_destroyed = cresetl.take_damage(50);
    println!(
        "{} {} destroyed",
        cresetl.name,
        if is_destroyed { "was" } else { "was not" }
    );
    cresetl.display_status();
    println!("\n\nLet's try again\n\n");
    is_destroyed = cresetl.take_damage(100);
    println!(
        "{} {} destroyed",
        cresetl.name,
        if is_destroyed { "was" } else { "was not" }
    );
    cresetl.display_status();

    println!("\n\n\nLet's try to refuel:");
    println!("Let's take some fuel first");
    cresetl.fuel -= 80;
    cresetl.display_status();
    println!("Let's refuel {}\n", cresetl.name);
    cresetl.refuel(3000);
    cresetl.display_status();

    println!("\n\nTesting cargo capabilities:\n\n");
    let mut cargo_loaded = cresetl.load_cargo(String::from("Bannana"));
    cresetl.display_status();

    while cargo_loaded {
        cargo_loaded = cresetl.load_cargo(String::from("Apple"));
    }
    println!("Loaded to max capacity");
    cresetl.display_status();
    println!("\n\nTrying to load one more: ");
    cargo_loaded = cresetl.load_cargo(String::from("Bee"));
    println!("Cargo was {} loaded", if cargo_loaded { "" } else { "not" });
    cresetl.display_status();

    println!("Clearing the cargo!!!\n\n");
    // creating a new instance of a new ship
    let mut krelens = Ship::new(String::from("Krelens"));
    cargo_loaded = krelens.load_cargo(String::from("Apple"));
    while cargo_loaded {
        cargo_loaded = krelens.load_cargo(String::from("Apple"));
    }
    let jetted_cargo = krelens.jettison_all_cargo();
    println!("Jetted all: {jetted_cargo:#?}");
    // Doesn't work as the krelens was dropped after jettison_all_cargo was called!
    // println!("{krelens:#?}");

    let mut my_ship = Ship::new(String::from("BorrowConflictTester"));
    let status = &my_ship;
    // my_ship.take_damage(10);
    // print!("{:#?}", status.name);
}

// Conceptual Questions
// Method Parameters
/*
    I would use &self whenever I create a simple method in impl block without the intention of moving the impl self
    I would use &mut self if to addition to the previous claim, I would like to alter some property of the implementing self
    I would use self when I want to mutate an object to a new object type and leave the old one unusable
*/
/*
    I would user struct over tuple if I need a name for the inner data of the object, otherwise, I could use a tuple
*/
/*
    I think I showed it clearly in the code
*/
