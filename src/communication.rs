use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait Communicator {
    async fn talk(&mut self, message: ChatMessage) -> String;
}

#[derive(Debug, Clone)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl<'de> Deserialize<'de> for MessageRole {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = String::deserialize(de)?;
        Ok(match variant.as_str() {
            "system" => MessageRole::System,
            "user" => MessageRole::User,
            "assistant" => MessageRole::Assistant,
            _other => MessageRole::Assistant,
        })
    }
}

impl Serialize for MessageRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(match *self {
            MessageRole::System => serializer.serialize_str("system")?,
            MessageRole::User => serializer.serialize_str("user")?,
            MessageRole::Assistant => serializer.serialize_str("assistant")?,
        })
    }
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
