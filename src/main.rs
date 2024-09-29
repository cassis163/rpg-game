use serde::{Deserialize, Serialize};

const LLM_API_URL: &str = "http://localhost:11434/api";

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    response: String,
}

async fn send_msg(http_client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let data: &str = r#"
        {
            "model": "game-llm",
            "prompt": "Why is the sky blue?",
            "stream": false
        }
        "#;
    let uri = format!("{}/generate", LLM_API_URL);
    let res = http_client
        .post(uri)
        .body(data)
        .send()
        .await?
        .json::<ChatResponse>()
        .await?;

    Ok(res.response)
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let client = reqwest::Client::new();
    
    let response = send_msg(&client).await.unwrap();
    println!("Response: {}", response);
}
