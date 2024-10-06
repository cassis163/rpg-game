use std::collections::HashMap;
use communication::Communicator;
use crate::item::{Item, ItemType};
use crate::player::Player;
use serde::{Deserialize, Serialize};
use crate::character::Character;
use serde_json_any_key::*;
use crate::Action::Give;
use crate::communication::{ChatMessage, MessageRole};
use crate::npc::Npc;

mod llm;
mod communication;
mod player;
mod character;
mod item;

mod npc;

// Describes an action a player or npc can perform. These are passed along inside the Interaction struct.
// For now only a give action exists. Some ideas:
// 1. NPCs can issue a 'Follow' action where they follow the player around until directed otherwise
// 2. 'Move' action where they can move to a certain location (Coordinates/named places) with path-finding.
// 3. 'Quest' action so that they can give out quests to players
// 4. 'Resupply': move to a certain location to stock up on new items (based on demand??!) if they run out
// 6. 'Open': open their shop/business on their preferred times
// 5. 'Close': close their shop/business
#[derive(Debug, Serialize, Deserialize)]
enum Action {
    Give {
        item: String,
        amount: i32,
    }
}

// What the Player sends to the model (+ the NpcContext below) and what the model returns to the player (only this)
#[derive(Debug, Serialize, Deserialize)]
struct Interaction {
    sender_id: String,
    receiver_id: String,
    message: String,
    actions: Vec<Action>,
}

// Provides the model (npc) with the state of the npc it is responding as
// For now only the items they have. But maybe things like their location, relations to other NPCs/players and more
#[derive(Serialize, Deserialize)]
struct NpcContext {
    npc_inventory: HashMap<Item, i32>,
}


#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut hank = Npc::new(&client, "Hank", "Blacksmith", "Hank is a well respected blacksmith in the Kingdom of Veldora").await;
    let mut pete = Npc::new(&client, "Pete", "Knight", "Pete is a fearless knight who has fought countless of great battles for the Kingdom of Veldora").await;
    let mut npcs: Vec<&mut Npc> = vec![&mut hank, &mut pete];

    let items: Vec<Item> = vec![
        Item::new("Gold Coin".to_string(), ItemType::Currency, "A round object made of pure gold. Used for buying and selling goods.".to_string(), ("Gold Coin".to_string(), 1)),
        Item::new("Steel Sword".to_string(), ItemType::Weapon, "A simple but trustworthy sword made of steel".to_string(), ("Gold Coin".to_string(), 5)),
    ];

    // Create a player to test with
    let mut bob = Player::new("Bob".to_string());
    // Give the player 500 Gold Coins to play with
    bob.add_item(items[0].clone(), 500);
    // Give the blacksmith npc 5 Steel Swords he can sell
    npcs[0].add_item(items[1].clone(), 5);

    let it = Interaction{
        sender_id: bob.name.to_string(),
        receiver_id: npcs[0].name.to_string(),
        message: "I'd like as many Steel Swords as this can get me and the change please".to_string(),
        actions: vec![Give {
            item: items[0].name.clone(),
            amount: 30,
        }],
    };

    let mut js = serde_json::to_string_pretty(&it).unwrap();
    js.push_str(npcs[0].get_items().to_json_map().unwrap().as_str());

    let cm = ChatMessage::new(MessageRole::User, js);
    println!("{}", npcs[0].talk(cm).await);
}