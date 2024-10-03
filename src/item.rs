#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Currency,
    Food,
    Weapon,
    Armor,
    Misc,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Item {
    // E.g. Sword
    pub(crate) name: String,
    // E.g. ItemType::Weapon
    pub(crate) item_type: ItemType,
    // E.g. A steel sword. Simple but trustworthy
    pub(crate) description: String,
    // E.g. Gold Coin, 1
    pub(crate) value: (String, i32),
}

impl Item {
    pub(crate) fn new(name: String, item_type: ItemType, description: String, value: (String, i32)) -> Item {
        Item {
            name,
            item_type,
            description,
            value
        }
    }
}