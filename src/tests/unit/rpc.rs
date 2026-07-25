use crate::rpc::client::{get_client, SupportedChains};
use alloy::primitives::{address, U256};
use alloy::providers::Provider;

#[tokio::test]
async fn test_sepolia_rpc_client_and_storage() {
    let client = get_client(&SupportedChains::Sepolia, None,None)
        .await
        .expect("Failed to initialize Sepolia RPC client");

    let contract_address = address!("0xF6446F446E56a73a2CE1660653BB1C44cF22ed8a");
    let slot = U256::from(0);

    let storage = client.get_storage_at(contract_address, slot)
        .await
        .expect("Failed to fetch storage from Sepolia");

    // The deployed contract on Sepolia has value 1 at slot 0
    assert_eq!(storage, U256::from(1));
}

#[test]
fn test_supported_chains_url_resolving() {
    let chain = SupportedChains::Sepolia;
    
    // Test resolving default RPC URL
    let resolved_default = chain.resolve_rpc_url(None);
    assert_eq!(resolved_default, "https://sepolia.drpc.org");

    // Test resolving custom RPC URL
    let custom_url = "https://custom-sepolia-endpoint.com/rpc";
    let resolved_custom = chain.resolve_rpc_url(Some(custom_url));
    assert_eq!(resolved_custom, custom_url);
}
