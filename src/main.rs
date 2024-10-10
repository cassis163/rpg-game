use std::collections::HashMap;
use communication::Communicator;
use crate::item::{Item};
use serde::{Deserialize, Serialize};
use serde_json_any_key::*;
use crate::character::CharacterTrait;

mod llm;
mod communication;
mod player;
mod character;
mod item;
mod npc;

mod app;
mod scene;

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


fn main() {
    app::launch_app();
}