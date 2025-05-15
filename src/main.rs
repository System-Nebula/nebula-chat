use anyhow::Result;
use iroh::protocol::Router;
use iroh::Endpoint;
use iroh_gossip::net::Gossip;


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
    router.shutdown().await?;

    Ok(())
}
