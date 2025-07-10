use crate::inventory::items::enums::ConsumableType;
use crate::inventory::items::enums::ItemType;
use crate::inventory::items::enums::MaterialType;
use crate::inventory::items::enums::ToolType;
use crate::inventory::items::enums::WeaponType;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Item {
    pub name: String,
    pub category: ItemType,
    pub weight: u32,
}

impl Item {
    pub fn new(name: String, category: ItemType) -> Self {
        Self {
            name,
            weight: match &category {
                ItemType::Tool(tool_type) => match tool_type {
                    ToolType::Axe => 2,
                    ToolType::Pickaxe => 1,
                    ToolType::Shovel => 3,
                },
                ItemType::Material(material_type) => match material_type {
                    MaterialType::Wood => 1,
                    MaterialType::Stone => 4,
                    MaterialType::IronOre => 5,
                    MaterialType::Stick => 2,
                    MaterialType::Log => 6,
                },
                ItemType::Consumable(consumable_type) => match consumable_type {
                    ConsumableType::Berry | ConsumableType::CookedMeat => 1,
                },
                ItemType::Weapon(weapon_type) => match weapon_type {
                    WeaponType::Sword => 5,
                    WeaponType::Bow => 4,
                },
            },
            category,
        }
    }
}
