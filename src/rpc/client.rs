//! RPC client module for initializing Alloy providers.
//!
//! Provides support for mapping supported EVM network chains to Alloy's
//! `NamedChain`, providing default RPC endpoints, resolving custom RPC URLs,
//! and building batching-enabled provider instances.

use std::time::Duration;

use alloy::eips::BlockId;
use alloy::providers::{layers::CallBatchLayer, Provider, ProviderBuilder};
use alloy::transports::TransportError;
use alloy_chains::NamedChain;
use alloy::transports::layers::RetryBackoffLayer;
use alloy::rpc::client::RpcClient;

/// List of supported EVM blockchain networks.
#[derive(Debug, PartialEq, Eq,Clone)]
pub enum SupportedChains {
    Mainnet,
    Sepolia,
    Arbitrum,
    ArbitrumSepolia,
    Base,
    BaseSepolia,
    Optimism,
    OptimismSepolia,
    Polygon,
    PolygonAmoy,
}

impl SupportedChains {
    /// Maps the `SupportedChains` enum variant to Alloy's built-in `NamedChain` type.
    pub const fn to_named_chain(&self) -> NamedChain {
        match self {
            SupportedChains::Mainnet => NamedChain::Mainnet,
            SupportedChains::Sepolia => NamedChain::Sepolia,
            SupportedChains::Arbitrum => NamedChain::Arbitrum,
            SupportedChains::ArbitrumSepolia => NamedChain::ArbitrumSepolia,
            SupportedChains::Base => NamedChain::Base,
            SupportedChains::BaseSepolia => NamedChain::BaseSepolia,
            SupportedChains::Optimism => NamedChain::Optimism,
            SupportedChains::OptimismSepolia => NamedChain::OptimismSepolia,
            SupportedChains::Polygon => NamedChain::Polygon,
            SupportedChains::PolygonAmoy => NamedChain::PolygonAmoy,
        }
    }

    /// Returns the default public HTTP RPC URL for the selected chain network.
    pub fn default_rpc_url(&self) -> &'static str {
        match self {
            SupportedChains::Mainnet => "https://eth.drpc.org",
            SupportedChains::Sepolia => "https://sepolia.drpc.org",
            SupportedChains::Arbitrum => "https://arbitrum.drpc.org",
            SupportedChains::ArbitrumSepolia => "https://sepolia-rollup.arbitrum.io/rpc",
            SupportedChains::Base => "https://mainnet.base.org",
            SupportedChains::BaseSepolia => "https://sepolia.base.org",
            SupportedChains::Optimism => "https://mainnet.optimism.io",
            SupportedChains::OptimismSepolia => "https://sepolia.optimism.io",
            SupportedChains::Polygon => "https://polygon-bor-rpc.publicnode.com",
            SupportedChains::PolygonAmoy => "https://rpc-amoy.polygon.technology",
        }
    }


/// Resolves which RPC URL to use for a connection.
///
/// Returns the custom `rpc_url` if provided in `Some(...)`,
/// otherwise falls back to the default RPC URL for the target `chain`.
pub fn resolve_rpc_url<'a>(&'a self, rpc_url: Option<&'a str>) -> &'a str {
    if let Some(url) = rpc_url {
        return url;
    }

    self.default_rpc_url()
}


}


/// Asynchronously builds and connects an Alloy HTTP RPC provider.
///
/// # Arguments
/// * `chain` - The target [`SupportedChains`] network.
/// * `rpc_url` - Optional custom RPC endpoint URL. If `None`, defaults to [`SupportedChains::default_rpc_url`].
///
/// # Features
/// - Configures the provider with the corresponding [`NamedChain`].
/// - Attaches a [`CallBatchLayer`] configured to buffer/batch JSON-RPC calls over a 10ms wait window.
///
/// # Errors
/// Returns a [`TransportError`] if connecting to the HTTP RPC endpoint fails.
pub async fn get_client(
    chain: &SupportedChains,
    rpc_url: Option<&str>,
    block_id: Option<u64>,
) -> Result<impl Provider, TransportError> {
    let retry_layer = RetryBackoffLayer::new(3, 100, 5);
    let url = chain.resolve_rpc_url(rpc_url);
    let parsed_url = url.parse().map_err(|e| {
        TransportError::local_usage_str(&format!("Invalid RPC URL: {}", e))
    })?;


    

   
let effective_block_id = block_id.map(|x| BlockId::number(x)).unwrap_or_else(BlockId::latest);


    let client=RpcClient::builder().layer(retry_layer).http(parsed_url);
    // Configure the batching layer with custom wait/batch parameters (10ms window)
    let provider = ProviderBuilder::new()
        .with_chain(chain.to_named_chain())
        .with_default_block(effective_block_id)
        .layer(CallBatchLayer::new().wait(Duration::from_millis(20)))
        .connect_client(client);
       

    Ok(provider)
}