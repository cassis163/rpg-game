use std::collections::HashMap;
use communication::Communicator;
use crate::communication::{ChatMessage, MessageRole};
use crate::item::{Item, ItemType};
use crate::npc::Npc;
use crate::player::Player;
use serde::{Deserialize, Serialize};

mod npc;
mod llm;
mod communication;
mod player;
mod character;
mod item;

#[derive(Debug, Serialize, Deserialize)]
enum Action {
    Give {
        item: String,
        amount: i32,
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Interaction {
    sender_id: String,
    receiver_id: String,
    message: String,
    actions: Vec<Action>,
}


#[tokio::main]
async fn main() {
    // let json = r#"
    // {
    //     "sender_id": "12345",
    //     "receiver_id": "54321",
    //     "message": "Hello, World!",
    //     "actions": [
    //         {
    //             "Give": {
    //                 "item": "Steel Sword",
    //                 "amount": 1
    //             }
    //         }
    //     ]
    // }
    // "#;
    //
    // let test = Interaction{
    //     sender_id: "12345".to_string(),
    //     receiver_id: "54321".to_string(),
    //     message: "Hello there".to_string(),
    //     actions: vec![
    //         ActionType::Give{item: "Gold Coin".to_string(), amount: 50},
    //     ],
    // };
    //
    // print!("{}", serde_json::to_string_pretty(&test).unwrap());
    //
    // dbg!(serde_json::from_str::<Interaction>(json).unwrap());

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
    bob.items = HashMap::new();
    // Give the player 500 Gold Coins to play with
    bob.items.insert(items[0].clone(), 500);
    // Give the blacksmith npc 5 Steel Swords he can sell
    npcs[0].items.insert(items[1].clone(), 5);

    loop {
        // Present the user with the available NPC's to talk with
        println!("Choose on of the following NPCs to talk to: ");
        let mut options = String::from("[");
        for npc in &npcs {
            options.push_str(&(npc.name));
            if npcs.iter().position(|n| n.name == npc.name) != Some(npcs.len() - 1) {
                options.push_str(", ");
            } else {
                options.push(']')
            }
        }
        println!("{}", options);

        // Get the user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Check if the npc exists (ignore case)
        if !npcs.iter().map(|n| n.name.to_lowercase()).collect::<Vec<String>>().contains(&&input.to_string().to_lowercase()) {
            println!("No NPC with name: {} exists!", input);
            continue;
        }

        // Get a mutable reference to the NPC so that we can alter his Message History and Inventory after interactions
        let npc = npcs.iter_mut().find(|n| n.name.to_lowercase() == input.to_lowercase()).unwrap();

        // talk to the chosen npc
        println!("Your message to {}: ", npc.name);
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let message = message.trim();

        // let mut user_to_npc = Interaction {
        //     sender_id: bob.name.clone(),
        //     receiver_id: npc.name.clone(),
        //     message: message.to_string(),
        //     actions: Vec::new(),
        // };

        let user_to_npc = serde_json::from_str::<Interaction>(message).unwrap();
        //dbg!(&user_to_npc);

        // Jsonify our interaction to send to our npc
        let js = serde_json::to_string_pretty(&user_to_npc).unwrap();

        println!("{}", npc.talk(ChatMessage::new(MessageRole::User, js)).await);
    }
}