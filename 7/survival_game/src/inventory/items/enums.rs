#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ToolType {
    Axe,
    Pickaxe,
    Shovel,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MaterialType {
    Wood,
    Stone,
    IronOre,
    Stick,
    Log,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ConsumableType {
    Berry,
    CookedMeat,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WeaponType {
    Sword,
    Bow,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ItemType {
    Tool(ToolType),
    Material(MaterialType),
    Consumable(ConsumableType),
    Weapon(WeaponType),
}
