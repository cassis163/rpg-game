use crate::communication::{ChatRequest, ChatResponse};

const LLM_API_URL: &str = "http://localhost:11434/api";

pub async fn send_msg(http_client: &reqwest::Client, request: &ChatRequest) -> Result<ChatResponse, reqwest::Error> {
    let uri = format!("{}/chat", LLM_API_URL);
    let res = http_client
        .post(uri)
        .json::<ChatRequest>(request)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    Ok(res)
}