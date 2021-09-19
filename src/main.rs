use ethers::core::types::H160;
use ethers::prelude::{Middleware, StreamExt};
use ethers::providers::Provider;
use ethers::utils::WEI_IN_ETHER;
use gumdrop::Options;
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
struct WatchAddress {
    name: String,
    address: H160,
}

#[derive(Serialize, Deserialize)]
struct Watchlist {
    watchlist: Vec<WatchAddress>,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Options, StructOpt)]
struct Opts {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    #[options(help = "The tracing / archival node's URL")]
    url: String,
}

static SOUND: &'static str = "Ping";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse_args_default_or_exit();
    let provider = Provider::connect(opts.url.as_str()).await.unwrap();

    let watchlist_data = fs::read_to_string("watchlist.json").expect("Unable to parse watchlist");
    let watchlist: Watchlist =
        serde_json::from_str(&watchlist_data).expect("JSON was not well-formatted");

    let mut address_names = HashMap::new();
    for wa in watchlist.watchlist.iter() {
        address_names.insert(wa.address, wa.name.clone());
    }

    let mut stream = provider.subscribe_blocks().await.unwrap();

    while let Some(item) = stream.next().await {
        let block = provider
            .get_block_with_txs(item.hash.unwrap())
            .await
            .unwrap();

        if block == None {
            println!("skipped block");
            continue;
        }
        for txn in block.unwrap().transactions {
            if address_names.contains_key(&txn.from)
                || (txn.to != None && address_names.contains_key(&txn.to.unwrap()))
            {
                let mut owner = address_names.get(&txn.from);
                if owner == None {
                    owner = address_names.get(&txn.to.unwrap())
                }

                println!(
                    "{0: <20} | {1: <90} | {2: <10}",
                    owner.unwrap(),
                    &format!("https://etherscan.io/tx/{:#x}", txn.hash),
                    txn.value / WEI_IN_ETHER,
                );

                Notification::new()
                    .summary(&format!(
                        "{} txn ({:.8})",
                        owner.unwrap(),
                        txn.value / WEI_IN_ETHER
                    ))
                    .action("open", "click to open")
                    .body(&format!("https://etherscan.io/tx/{:#x}", txn.hash))
                    .timeout(10)
                    .sound_name(SOUND)
                    .show()?;
            }
        }
    }

    Ok(())
}
