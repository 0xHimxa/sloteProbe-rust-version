use std::time::Duration;

use alloy::providers::{Provider, ProviderBuilder,layers::CallBatchLayer};
use alloy_chains::NamedChain;
use alloy::transports::TransportError;


#[derive(Debug,PartialEq,Eq)]

pub enum SupportedChains{
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



impl SupportedChains{


//Maps the custom enum  to Alloys built-in NamedChain

 pub const fn to_named_chain(&self) -> NamedChain{

  match self{

    SupportedChains::Mainnet=>NamedChain::Mainnet,
    SupportedChains::Sepolia=>NamedChain::Sepolia,
    SupportedChains::Arbitrum=>NamedChain::Arbitrum,
    SupportedChains::ArbitrumSepolia=>NamedChain::ArbitrumSepolia,
    SupportedChains::Base=>NamedChain::Base,
    SupportedChains::BaseSepolia=>NamedChain::BaseSepolia,
    SupportedChains::Optimism=>NamedChain::Optimism,
    SupportedChains::OptimismSepolia=>NamedChain::OptimismSepolia,
    SupportedChains::Polygon=>NamedChain::Polygon,
    SupportedChains::PolygonAmoy=>NamedChain::PolygonAmoy,
  }


 }






 pub fn default_rpc_url(&self) -> &'static str {
        match self {
            SupportedChains::Mainnet => "https://eth.drpc.org",
            SupportedChains::Sepolia => "https://rpc.ankr.com/eth_sepolia",
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







 






}




pub fn reslove_rpc_url<'a>(chain:&SupportedChains,rpc_url:Option<&'a str>)->&'a str{

  if let Some(url) = rpc_url{
    return url
  }

  chain.default_rpc_url()

}




pub async fn get_client(chain:&SupportedChains,rpc_url:Option<&str>) -> Result<impl Provider,TransportError>{
    // Configure the batching layer with custom wait/batch parameters
   

  let provider =  ProviderBuilder::new().with_chain(chain.to_named_chain()).layer(CallBatchLayer::new()
        .wait(Duration::from_millis(10))).connect(reslove_rpc_url(chain,rpc_url)).await?;
   
Ok(provider)

}