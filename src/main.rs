mod core;
mod rpc;

#[cfg(test)]
mod tests;

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
let client = get_client(&SupportedChains::Sepolia, None).await.unwrap();

let storage=client.get_storage_at(address!("0xF6446F446E56a73a2CE1660653BB1C44cF22ed8a"),U256::from(0)).await.unwrap();
print!("{}",storage);


}
