use anyhow::Result;
use futures_lite::StreamExt;
use iroh::protocol::Router;
use iroh::{Endpoint, NodeId};
use iroh_gossip::net::{Event,Gossip,GossipEvent,GossipReceiver};
use iroh_gossip::proto::TopicId;
use serde::{Deserialize,Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    body: MessageBody,
    nonce: [u8; 16],
}

#[derive(Debug, Serialize, Deserialize)]
enum MessageBody {
    AboutMe {from: NodeId,name: String},
    Message { from: NodeId, text: String},
}

impl Message {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }
    pub fn new(body: MessageBody) -> Self {
        Self { body , nonce: rand::random(), }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

async fn subscribe_loop(mut receiver: GossipReceiver) -> Result<()>{
    let mut names = HashMap::new();

    while let Some(event) = receiver.try_next().await? {

        if let Event::Gossip(GossipEvent::Received(msg)) = event {

            match Message::from_bytes(&msg.content)?.body {

                MessageBody::AboutMe { from, name } => {
                    names.insert(from,name.clone());
                    println!("> {} is now known as {}",from.fmt_short(), name);
                }

                MessageBody::Message { from, text } => {
                    let name = names
                    .get(&from)
                    .map_or_else(|| from.fmt_short(), String::to_string);
                    println!("{}: {}", name, text);
                }
            }
        } 
    }
    Ok(())
}

fn input_loop(line_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = std::io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        line_tx.blocking_send(buffer.clone())?;
        buffer.clear();
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::builder()
        .discovery_n0()
        .bind()
        .await?;
    print!("> node id: {}", endpoint.node_id());

    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    let id = TopicId::from_bytes(rand::random());
    let node_ids = vec![];
    let topic = gossip.subscribe(id, node_ids)?;
    // Splitting the topic into a Gossip sender and a Gossip receiver
    let (sender, receiver) = topic.split();
    let message = Message::new(MessageBody::AboutMe {
        from: endpoint.node_id(),
        name: String::from("franky"),
    });
    sender.broadcast(message.to_vec().into()).await?;
    tokio::spawn(subscribe_loop(receiver));
    let (line_tx, mut line_rx) = tokio::sync::mpsc::channel(1);
    std::thread::spawn(move || input_loop(line_tx));

    println!("> type a message and hit enter to broadcast...");
    while let Some(text) = line_rx.recv().await {
        let message = Message::new(MessageBody::Message {
            from: endpoint.node_id(),
            text: text.clone(), 
        });
        sender.broadcast(message.to_vec().into()).await?;
        println!("> me: {text}");
    }

    router.shutdown().await?;

    Ok(())
}
