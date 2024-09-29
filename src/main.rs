use communication::Communicator;

mod npc;
mod llm;
mod communication;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let npc = npc::Npc::new("Hank", &client);
    npc.talk("Hello").await;
}
