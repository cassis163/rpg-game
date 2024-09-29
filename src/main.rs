use communication::Communicator;

mod npc;
mod llm;
mod communication;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut hank = npc::Npc::new("Hank", &client);
    let mut pete = npc::Npc::new("Pete", &client);

    while true {
        // choose to talk to hank or pete
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let npc = if input == "hank" {
            &mut hank
        } else if input == "pete" {
            &mut pete
        } else {
            println!("Invalid input");
            continue;
        };
        
        // talk to the chosen npc
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        let message = message.trim();
        npc.talk(message).await;
    }
}
