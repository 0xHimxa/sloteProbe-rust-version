mod core;
mod rpc;

use rpc::client::*;
 

use core::artifact_parser::foundry::ArtifactFile;

use alloy::providers::Provider;
use alloy::eips::BlockNumberOrTag;
use alloy::primitives::{Address,address,U256};


#[tokio::main]
async fn main(){

  
//   let artifact_file=ArtifactFile::open("t2.json").unwrap();
//   let foundry_raw_layout=artifact_file.load_foundry_artifact().unwrap();
//   let normalized=ArtifactFile::normalize_artifacts(&foundry_raw_layout).unwrap();
//   println!("{:#?}",normalized);
let client = get_client(&SupportedChains::BaseSepolia, None).await.unwrap();

let storage=client.get_storage_at(address!("0xf04E1047F34507C7Cf60fDc811116Bc7b0E923f3"),U256::from(2)).await.unwrap();
print!("{}",storage);


}
