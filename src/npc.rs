use crate::{communication::{Communicator, Context}, llm::{ChatRequest, send_msg}};

pub struct Npc<'a> {
    name: String,
    context: Context,
    http_client: &'a reqwest::Client,
}

impl Npc<'_> {
    pub fn new<'a>(name: &str, http_client: &'a reqwest::Client) -> Npc<'a> {
        Npc {
            name: name.to_string(),
            context: Context::new(),
            http_client,
        }
    }
}

impl Communicator for Npc<'_> {
    async fn talk(&mut self, message: &str) -> String {
        let request = ChatRequest::new(message, &self.context);
        let response = send_msg(self.http_client, &request).await.unwrap();
        &self.context.set_context(response.get_context());
        print!("{}: {}\n", self.name, response.get_response());
        return response.get_response();
    }
}