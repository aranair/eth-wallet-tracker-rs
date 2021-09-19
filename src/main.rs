use ethers::core::types::Address;
use std::convert::TryFrom;

use ethers::core::types::{Block, TxHash, H160, U256};
use ethers::core::types::{Filter, ValueOrArray};
use std::collections::HashMap;

use ethers::prelude::{JsonRpcClient, Log, Middleware, PubsubClient, StreamExt};
use ethers::providers::{Http, Provider, Ws};

use gumdrop::Options;
use structopt::StructOpt;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse_args_default_or_exit();
    let provider = Provider::connect(opts.url.as_str()).await.unwrap();

    let tetranode: H160 = "0x9c5083dd4838e120dbeac44c052179692aa5dac5".parse()?;
    let jebus911: H160 = "0xEF30fA2138A725523451688279b11216B0505E98".parse()?;
    let sam: H160 = "0x84D34f4f83a87596Cd3FB6887cFf8F17Bf5A7B83".parse()?;
    let mevcollector: H160 = "0x5338035c008EA8c4b850052bc8Dad6A33dc2206c".parse()?;
    let crypto888crypto: H160 = "0x0a2542a170aA02B96B588aA3AF8B09AB22a9D7ac".parse()?;
    let defigod: H160 = "0x3F3E305C4Ad49271EBDA489dd43d2c8F027d2D41".parse()?;
    let vvd: H160 = "0x0F0eAE91990140C560D4156DB4f00c854Dc8F09E".parse()?;
    let address9: H160 = "0xd5a9C4a92dDE274e126f82b215Fccb511147Cd8e".parse()?;

    let mut address_names = HashMap::new();
    address_names.insert(tetranode, "tetranode".to_string());
    address_names.insert(jebus911, "jebus911".to_string());
    address_names.insert(sam, "sam".to_string());
    address_names.insert(mevcollector, "mevcollector".to_string());
    address_names.insert(crypto888crypto, "crypto888crypto".to_string());
    address_names.insert(defigod, "defigod".to_string());
    address_names.insert(vvd, "vvd".to_string());
    address_names.insert(address9, "0x541nt".to_string());

    let filter = Filter::new();
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
                    "{} --- https://etherscan.io/tx/{:#x}",
                    owner.unwrap(),
                    txn.hash
                );
            }
        }
    }

    Ok(())
}
