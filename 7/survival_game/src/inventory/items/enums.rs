#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ToolType {
    Axe,
    Pickaxe,
    Shovel,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MaterialType {
    Wood,
    Stone,
    IronOre,
    Stick,
    Log,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConsumableType {
    Berry,
    CookedMeat,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WeaponType {
    Sword,
    Bow,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    Tool(ToolType),
    Material(MaterialType),
    Consumable(ConsumableType),
    Weapon(WeaponType),
}
