use crate::{talk::{Talk, Context}, llm::{ChatRequest, send_msg}};

pub struct Npc<'a> {
    pub name: String,
    context: Context,
    http_client: &'a reqwest::Client,
}

impl Npc<'_> {
    pub fn new<'a>(name: &str, http_client: &'a reqwest::Client) -> Npc<'a> {
        Npc {
            name: name.to_string(),
            context: Context::default(),
            http_client,
        }
    }
}

impl Talk for Npc<'_> {
    async fn talk(&mut self, message: &str) -> String {
        let request = ChatRequest::new(message, &self.context);
        let chat_response = send_msg(self.http_client, &request).await.unwrap();
        self.context.context_parameter = Some(chat_response.context);
        chat_response.response
    }
}