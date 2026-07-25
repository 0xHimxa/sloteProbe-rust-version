

use alloy::primitives::{ U256,B256,Address};
use alloy::providers::Provider;
use crate::rpc::client::{get_client, SupportedChains};

pub async fn read_slot(address:Address,slot:U256,block_number:u64, chain:SupportedChains, rpc_url:Option<&str>) -> B256{

    let block_id:Option<u64> = (!block_number == 0).then_some(block_number);
    
let client=get_client(&chain, rpc_url,block_id).await.unwrap();

let value :B256= client.get_storage_at(address,slot).await.unwrap().into();

value

}