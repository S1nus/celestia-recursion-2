use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

use tendermint_light_client_verifier::{
    options::Options, types::LightBlock, ProdVerifier, Verdict, Verifier,
};

/// Celestia header scraper
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to store downloaded headers
    #[arg(short, long, value_name = "PATH")]
    output_path: PathBuf,
    
    /// Tendermint RPC URL
    #[arg(short, long, value_name = "URL", default_value = "http://localhost:26657")]
    rpc_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Celestia Scraper starting...");
    println!("Headers will be stored in: {:?}", args.output_path);
    println!("Using RPC URL: {}", args.rpc_url);
    
    let client = celestia_recursion::tm_rpc_utils::TendermintRPCClient::new(args.rpc_url);

    let peer_id = client.fetch_peer_id().await.unwrap();

    let genesis_block = client.fetch_light_block(1, peer_id).await.unwrap();

    let latest_block_height = client.get_latest_block_height().await;
    let latest_block = client.fetch_light_block(latest_block_height, peer_id).await.unwrap();

    let vp = ProdVerifier::default();
    let opt = Options {
        trust_threshold: Default::default(),
        // 2 week trusting period.
        trusting_period: Duration::from_secs(14 * 24 * 60 * 60),
        clock_drift: Default::default(),
    };

}
