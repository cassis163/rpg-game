mod npc;
mod llm;
mod app;
mod scene;
mod player;
mod character;

#[tokio::main]
async fn main() {
    // let client = reqwest::Client::new();
    // let mut hank = npc::Npc::new("Hank", &client);
    // let mut pete = npc::Npc::new("Pete", &client);

    app::launch_app();
}
