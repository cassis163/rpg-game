use std::collections::HashMap;
use std::vec;
use crate::{communication::{Communicator}, llm::{send_msg}};
use crate::character::Character;
use crate::communication::{ChatMessage, ChatRequest, MessageRole};
use crate::item::Item;
use crate::player::Player;

#[derive(Clone)]
pub struct Npc<'a> {
    http_client: &'a reqwest::Client,
    pub(crate) message_history: Vec<ChatMessage>,
    pub(crate) name: String,
    pub(crate) occupation: String,
    pub(crate) backstory: String,
    pub(crate) items: HashMap<Item, i32>,
}

impl Npc<'_> {
    pub(crate) async fn new<'a>(http_client: &'a reqwest::Client, name: &str, occupation: &str, backstory: &str) -> Npc<'a> {
        let mut npc_init_msg = ""; //"This is an RPG game and you are responsible for handling the NPC dialogue. The user will send requests in this format: PLAYER [PLAYER_NAME] SAYS [MESSAGE] TO NPC [NPC_NAME] or to give an item like this: PLAYER [PLAYER_NAME] GIVES ITEM [ITEM_AMOUNT] [ITEM_NAME] TO NPC [NPC_NAME]. You must respond in this exact same format as well. For example: NPC [NPC_NAME] SAYS [MESSAGE] TO PLAYER [PLAYER_NAME] or to give an item to the player: NPC [NPC_NAME] GIVES ITEM [ITEM_AMOUNT] TO PLAYER [PLAYER_NAME]. Instead of saying you give an item to the player actually give the item to the player using the format mentioned before! Always follow the format! Variablese like the content of a message, the amount of items you give and the item name all go inside of square brackets: []! Make sure to always put TO PLAYER [PLAYER_NAME] after each command otherwise the game will not function. This is very important make sure to do this at all times! Some more examples with variable names filled in: NPC [Hank] SAYS [Hi there Bob] TO PLAYER [Bob]. NPC [Hank] GIVES ITEM [1] [Steel Sword] TO PLAYER [Bob]".to_string();
        let test = format!("You are a NPC in a RPG game. Your name is {name} and you are a {occupation}. This is your backstory: {backstory}.");
        Npc {
            http_client,
            message_history: vec![ChatMessage::new(MessageRole::System, test)],
            name: name.to_string(),
            occupation: occupation.to_string(),
            backstory: backstory.to_string(),
            items: HashMap::new(),
        }
    }
}


impl Character for &mut Npc<'_> {
    fn set_items(&mut self, items: HashMap<Item, i32>) {
        self.items = items;
    }

    fn add_item(&mut self, item: Item, amount: i32) {
        for (key, mut value) in &mut self.items {
            if key.name == item.name {
                *value += amount;
                return;
            }
        }
        self.items.insert(item, amount);
    }

    //noinspection DuplicatedCode
    fn remove_item(&mut self, item: Item, amount: i32) -> bool {
        // If player does not have the item, return false
        if self.items.iter().position(|(key, value)| key.name == item.name).is_none() {
            return false;
        }
        self.items = self.items.clone().into_iter().filter_map(|(key, value)| {
            if key.name != item.name {
                return Some((key.to_owned(), value));
            }
            if value - amount <= 0 {
                return None;
            }
            Some((key.to_owned(), value - amount))
        }).collect();
        true
    }

    fn get_items(&self) -> &HashMap<Item, i32> {
        &self.items
    }

    fn print_self(&self) {
        println!("{}", self.name);
        for (item, amount) in &self.items {
            println!("{}: {}", item.name, amount);
        }
    }
}

impl Communicator for Npc<'_> {
    async fn talk(&mut self, message: ChatMessage) -> String {
        // Push user's message into the history
        self.message_history.push(message);
        let request = ChatRequest::new(self.message_history.clone());
        let response = send_msg(self.http_client, &request).await.unwrap();
        // Push models response message into the history
        self.message_history.push(response.get_message());
        // Return the response message
        response.get_message().get_content()
    }
}