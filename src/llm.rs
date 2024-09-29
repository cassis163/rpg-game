use serde::{Deserialize, Serialize};

use crate::communication::Context;

const LLM_API_URL: &str = "http://localhost:11434/api";

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    model: String,
    prompt: String,
    stream: bool,
    context: Option<Vec<i32>>,
}

impl ChatRequest {
    pub fn new(prompt: &str, context: &Context) -> ChatRequest {
        ChatRequest {
            model: "game-llm".to_string(),
            prompt: prompt.to_string(),
            stream: false,
            context: context.get_context(),
        }
    }
}

pub async fn send_msg(http_client: &reqwest::Client, request: &ChatRequest) -> Result<String, reqwest::Error> {
    let uri = format!("{}/generate", LLM_API_URL);
    let res = http_client
        .post(uri)
        .json::<ChatRequest>(request)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    Ok(res.response)
}