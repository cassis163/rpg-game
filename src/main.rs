use std::collections::HashMap;
use std::str::FromStr;
use communication::Communicator;
use crate::character::Character;
use crate::communication::{ChatMessage, MessageRole};
use crate::item::{Item, ItemType};
use crate::npc::Npc;
use crate::player::Player;
use fancy_regex::Regex;

mod npc;
mod llm;
mod communication;
mod player;
mod character;
mod item;

#[tokio::main]
async fn main() {

    //parse_interaction("NPC [Herald] GIVES ITEM [5] [Gold Coin] TO PLAYER [Bob]".to_string());
    //parse_interaction("NPC [Herald] SAYS [Hello] TO PLAYER [Bob]".to_string());

    let client = reqwest::Client::new();
    let mut hank = npc::Npc::new(&client, "Hank", "Blacksmith", "Hank is a well respected blacksmith in the Kingdom of Veldora").await;
    let mut pete = npc::Npc::new(&client, "Pete", "Knight", "Pete is a fearless knight who has fought countless of great battles for the Kingdom of Veldora").await;
    let mut npcs: Vec<&mut npc::Npc> = vec![&mut hank, &mut pete];

    let items: Vec<Item> = vec![
        Item::new("Gold Coin".to_string(), ItemType::Currency, "A round object made of pure gold. Used for buying and selling goods.".to_string(), ("Gold Coin".to_string(), 1)),
        Item::new("Steel Sword".to_string(), ItemType::Weapon, "A simple but trustworthy sword made of steel".to_string(), ("Gold Coin".to_string(), 5)),
    ];

    let mut bob = Player::new("Bob".to_string());
    bob.items = HashMap::new();
    bob.items.insert(items[0].clone(), 500);
    npcs[0].items.insert(items[1].clone(), 5);
    npcs[0].print_self();


    loop {
        // choose to talk to hank or pete
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
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !npcs.iter().map(|n| &n.name).collect::<Vec<&String>>().contains(&&input.to_string()) {
            println!("No NPC with name: {} exists!", input);
            continue;
        }

        let npc = npcs.iter().find(|n| n.name == input).unwrap();
        println!("Your message to {}: ", npc.name);

        // talk to the chosen npc
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let message = message.trim();
        let user_interaction = parse_interaction(message.to_string());
        let npc_clone = npcs[0].clone();
        //let res = npcs[0].talk(ChatMessage::new(MessageRole::User, mansplain_ai(&u_interaction, &npc_clone))).await;
        //execute_interaction_result(n_interaction, &mut bob, &mut npcs, &items);
    }
}

fn mansplain_ai(interaction: &Option<InteractionResult>, npc: &Npc) -> String {
    // The AI is stoopid stinky and needs some help choosing logical responses and also sticking to the damn format :D
    let mut res = String::new();

    // For actions
    if let Some(InteractionResult::ItemTransfer(sndr, rcvr, amount, item)) = interaction {
        // Whether the NPC is receiving or sending an item
        let mut receiving = true;
        if *sndr == npc.name {
            receiving = false;
        }

        res.push_str(format!("

Action: The player {sndr} has just given you {amount} {item}

If this is a gift from the player, you must respond in this exact format (keeping the brackets around variables):

NPC [NPC_NAME] SAYS [MESSAGE] TO PLAYER [PLAYER_NAME]

You must keep the brackets around the variables when responding. Replace the words inside the brackets with the correct values, but do not remove the brackets.

If this is part of a deal or trade, follow these steps:

    Validate the trade:
        Check if the item given matches the expected item for the trade.
        Check if the amount matches the required amount.

    If the trade is successful, respond in this exact format:

NPC [NPC_NAME] GIVES ITEM [ITEM_AMOUNT] [ITEM_NAME] TO PLAYER [PLAYER_NAME]

or you could say something to the player and give the item by chaining the commands with && like this for example:
NPC [NPC_NAME] SAYS [MESSAGE] TO PLAYER [PLAYER_NAME] && NPC [NPC_NAME] GIVES ITEM [ITEM_AMOUNT] [ITEM_NAME] TO PLAYER [PLAYER_NAME]

You must keep the brackets around the variables in the response. Replace the words inside the brackets with the correct values, but do not remove the brackets.

    If the trade is not successful (e.g., wrong item or amount), respond in this exact format:

NPC [NPC_NAME] SAYS [MESSAGE] TO PLAYER [PLAYER_NAME]

Be sure to explain why the trade is not successful, but make sure that the response keeps the brackets around variables.

IMPORTANT:
    Keep the brackets [ ] in the response around the variables: [PLAYER_NAME], [NPC_NAME], [ITEM_AMOUNT], [ITEM_NAME], [MESSAGE].
    Replace only the values inside the brackets.
    Ensure the format remains exactly as shown, with the brackets intact.").as_str());

        // Tell the ai what items they have
        let mut npc_items = String::from("These are the items you have on you now: \n");

        for (item, amount) in &npc.items {
            npc_items.push_str(format!("{}: {}\n", item.name, amount).as_str());
        }

        res.push_str("Make sure to only give items and amounts that you have. The game will make sure to update your inventory for you so you don't have to worry about that yourself");

        return res;
    } // For messages
    else if let Some(InteractionResult::Message(player_name, msg)) = interaction {
        res.push_str(format!("\
        PLAYER {player_name} has just sent you a message: {msg}.
        You can respond to this by sending back a message exactly like this:
        NPC [NPC_NAME] SAYS [MESSAGE] TO PLAYER [PLAYER_NAME]
        Where everything inside of the square brackets must be replaced by the correct variables. Make sure to keep the brackets though. Only change what's inside!
        Also you must include TO PLAYER [PLAYER_NAME] at the end otherwise it won't work
        ").as_str());
        // Tell the ai what items they have
        let mut npc_items = String::from("These are the items you have on you now: \n");

        for (item, amount) in &npc.items {
            npc_items.push_str(format!("{}: {}\n", item.name, amount).as_str());
        }
        res.push_str(npc_items.as_str());
        res.push_str("If the player asks to see what items you have you can only respond with the items that are listed above");
        return res;
    }
    "Something went wrong. Do not respond to this message!".to_string()
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum InteractionResult {
    // SENDER RECEIVER ITEM_AMOUNT ITEM_NAME
    ItemTransfer(String, String, i32, String),
    // PLAYER MESSAGE
    Message(String, String),
}

fn parse_interaction(res: String) -> String {
    // PLAYER [Bob] SAYS [Hello] TO NPC [Herald] --> No actions involved we can ignore
    // PLAYER [Bob] GIVES ITEM [1] [Gold Coin] TO NPC [Herald]
    // NPC [Herald] GIVES ITEM [1] [Steel Sword] TO PLAYER [Bob]
    // NPC [Herald] SAYS [Here you go!] TO PLAYER [Bob] && NPC [Herald] GIVES ITEM [1] [Steel Sword] TO PLAYER [Bob]

    let npc_name_pattern = Regex::new(r"NPC (\w+)(?![\]])").unwrap(); // Match NPC name without closing bracket
    let player_name_pattern = Regex::new(r"PLAYER (\w+)(?![\]])").unwrap(); // Match PLAYER name without closing bracket
    let item_pattern = Regex::new(r"ITEM (\d+)(?![\]]) (.+?)(?![\]]) TO").unwrap(); // Capture item amount and name
    let message_pattern = Regex::new(r"SAYS \[?(.+?)\]? TO").unwrap(); // Match message with optional brackets

    let mut interactions: Vec<InteractionResult> = Vec::new();

    let commands = res.split("&&").collect::<Vec<_>>();

    for command in commands {
        let mut sender = "NULL";
        let mut receiver = "NULL";

        let mut pos: usize = command.find("PLAYER").unwrap();

        let player_name: String = find_next_value(command.to_string(), &mut pos).unwrap();
        pos = command.find("NPC").unwrap();
        let npc_name: String = find_next_value(command.to_string(), &mut pos).unwrap();

        // Look for action keywords (in this case only GIVES)
        if !command.contains("GIVES") && command.contains("SAYS") {
            println!("No action detected. Skipping action check!");
            interactions.push(InteractionResult::Message(player_name.clone(), res.to_string()));
            continue;
        }

        if command.find("PLAYER") < command.find("NPC") {
            sender = player_name.as_str();
            receiver = npc_name.as_str();
        } else {
            sender = npc_name.as_str();
            receiver = player_name.as_str();
        }

        // Iterate through the interaction until we find ITEM keyword
        pos = command.find("ITEM").unwrap();
        let amount: i32 = find_next_value(command.to_string(), &mut pos).unwrap();
        // Extract the [ITEM_NAME]
        let item_name: String = find_next_value(command.to_string(), &mut pos).unwrap();
        interactions.push(InteractionResult::ItemTransfer(sender.to_string(), receiver.to_string(), amount, item_name));
    }
    // Hopefully actions come first this way:
    interactions.sort();

    
    res
}


fn fix_npc_format(response: &str) -> String {
    // Define regex patterns to extract NPC name, player name, item, and item amount
    let npc_name_pattern = Regex::new(r"NPC (\w+)(?![\]])").unwrap(); // Match NPC name without closing bracket
    let player_name_pattern = Regex::new(r"PLAYER (\w+)(?![\]])").unwrap(); // Match PLAYER name without closing bracket
    let item_pattern = Regex::new(r"ITEM (\d+)(?![\]]) (.+?)(?![\]]) TO").unwrap(); // Capture item amount and name
    //let message_pattern = Regex::new(r"SAYS (.+?)(?![\]]) TO").unwrap(); // Capture message
    let message_pattern = Regex::new(r"SAYS \[?(.+?)\]? TO").unwrap(); // Match message with optional brackets

    // Extract NPC name
    let npc_name = if let Ok(Some(caps)) = npc_name_pattern.captures(response) {
        format!("[{}]", &caps[1])
    } else {
        "[NPC_NAME]".to_string()
    };

    // Extract player name
    let player_name = if let Ok(Some(caps)) = player_name_pattern.captures(response) {
        format!("[{}]", &caps[1])
    } else {
        "[PLAYER_NAME]".to_string()
    };

    // Extract item amount and name
    let (item_amount, item_name) = if let Ok(Some(caps)) = item_pattern.captures(response) {
        let amount = format!("[{}]", &caps[1]);
        let item_name = format!("[{}]", &caps[2]);
        (amount, item_name)
    } else {
        ("[ITEM_AMOUNT]".to_string(), "[ITEM]".to_string())
    };

    // Extract message (if applicable)
    let message = if let Ok(Some(caps)) = message_pattern.captures(response) {
        format!("[{}]", &caps[1])
    } else {
        "[MESSAGE]".to_string()
    };


    // Build the corrected response by dynamically replacing the extracted parts
    let mut processed_response = response.to_string();


    // Replace NPC name
    if let Ok(Some(caps)) = npc_name_pattern.captures(response) {
        processed_response = processed_response.replace(format!("NPC {}", &caps[1]).as_str(), format!("NPC {}", &npc_name).as_str());
    }

    // Replace player name
    if let Ok(Some(caps)) = player_name_pattern.captures(response) {
        processed_response = processed_response.replace(&caps[1], &player_name);
    }

    // Replace the item and amount in the processed_response
    if let Ok(Some(caps)) = item_pattern.captures(response) {
        processed_response = processed_response.replace(&caps[1], &item_amount);
        processed_response = processed_response.replace(&caps[2], &item_name);
    }

    // Replace message
    if let Ok(Some(caps)) = message_pattern.captures(response) {
        processed_response = processed_response.replace(&caps[1], &message);
    }


    // Return the processed response
    processed_response
}

fn find_next_value<T: FromStr>(res: String, mut pos: &mut usize) -> Option<T> {
    let mut in_brackets = false;
    let mut value_str: String = String::new();
    for c in res.to_string()[pos.clone()..].chars() {
        *pos += 1;
        if c == '[' {
            in_brackets = true;
            continue;
        } else if c == ']' {
            break;
        } else if in_brackets {
            value_str.push(c);
        }
    }
    if let Ok(parsed) = value_str.parse::<T>() {
        return Some(parsed);
    }
    None
}

fn execute_interaction_result(result: &Option<InteractionResult>, bob: &mut Player, npcs: &mut Vec<&mut Npc>, items: &Vec<Item>) {
    if let Some(InteractionResult::ItemTransfer(sndr, rcvr, amount, item)) = result {
        //println!("{sndr} {rcvr} {amount} {item}");
        let mut sender: &mut dyn Character;
        let mut receiver: &mut dyn Character;

        if *sndr == bob.name {
            sender = bob;
            receiver = &mut npcs[0];
        } else {
            sender = &mut npcs[0];
            receiver = bob;
        }

        if let Some(actual_item) = items.iter().find(|i| i.name == *item) {
            if sender.remove_item(actual_item.clone(), *amount) {
                receiver.add_item(actual_item.clone(), *amount);
            } else {
                println!("Sender did not have the item(s) for this transfer");
            }
        } else {
            println!("Item does not exist in the world!");
        }
        println!("Sender items: ");
        for (item, amount) in sender.get_items() {
            println!("{}: {}", item.name, amount);
        }
        println!("Receiver items: ");
        for (item, amount) in receiver.get_items() {
            println!("{}: {}", item.name, amount);
        }
    }
    if let Some(InteractionResult::Message(sndr, message)) = result {
        println!("{}", message);
    }
}
