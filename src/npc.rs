use std::collections::HashMap;
use std::vec;
use crate::{communication::{Communicator}, llm::{send_msg}};
use crate::character::Character;
use crate::communication::{ChatMessage, ChatRequest, MessageRole};
use crate::item::Item;

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
        let npc_init = format!("You are a NPC in a RPG game. Your name is {name} and you are a {occupation}. This is your backstory: {backstory}.\n\
        The communication between you as a npc and the player will be done using json objects. This is generally how one would look: \n{}, \n{}\n{}]n
        ", r#"
        {
            "sender_id": "Bob",
            "receiver_id": "Hank",
            "message": "Deal! 50 Gold Coins for a Steel Sword sounds good to me.",
            "actions": [
                {
                    "Give":
                    {
                        "item": "Gold Coin",
                        "amount": 50
                    }
                }
            ]
        }
        "#, "
         You have to replace the values for these keys with the appropriate values. For example in the example above a player agrees to buy a Steel Sword from you for 50 Gold Coins.\n\
         In the message he lets this know and in the list of actions he triggers the Give action with the parameters specifying which item he sends to you and the amount of items.
         You would respond to this with a message to your liking and a Give action as well. For example:
        ", r#"
        {
        "sender_id": "Hank",
        "receiver_id": "Bob",
        "message": "It was a pleasure doing business with you!",
        "actions": [
            {
                "Give":
                {
                    "item": "Steel Sword",
                    "amount": 1
                }
            }
        ]
        }
        "#);
        Npc {
            http_client,
            message_history: vec![ChatMessage::new(MessageRole::System, npc_init)],
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
        for (key, value) in &mut self.items {
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
        if self.items.iter().position(|(key, _value)| key.name == item.name).is_none() {
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