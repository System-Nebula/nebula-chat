use anyhow::Result;
use iroh::{Endpoint,SecretKey};


#[tokio::main]
async fn main() -> Result<()> {
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    let endpoint = Endpoint::builder().secret_key(secret_key)
        .discovery_n0()
        .bind()
        .await?;
    print!("> node id: {}", endpoint.node_id());
    Ok(())

}
