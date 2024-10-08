use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait Communicator {
    async fn talk(&mut self, message: ChatMessage) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    role: MessageRole,
    content: String,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: String) -> ChatMessage {
        ChatMessage { role, content }
    }
    pub fn get_role(&self) -> MessageRole {
        self.role.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    message: ChatMessage,
}

impl ChatResponse {
    pub fn get_message(&self) -> ChatMessage {
        self.message.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

impl ChatRequest {
    pub fn new(messages: Vec<ChatMessage>) -> ChatRequest {
        ChatRequest {
            model: "game-llm".to_string(),
            messages,
            stream: false,
        }
    }
}