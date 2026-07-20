use serde::{Deserialize,Serialize};
use alloy::primitives::U256;
use std::collections::HashMap;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryStorage{
   pub label: String,
   pub offset: u8,
   pub slot:String,
   #[serde(rename = "type")]
   pub type_of: String,
   
}


pub type FoundryTypeRaw = HashMap<String,FoundryTypeInfo>;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryTypeInfo{
pub encoding:String,
#[serde(rename = "numberOfBytes")]
pub number_of_bytes:String,
pub label:Option<String>,
pub members:Option<Vec<FoundryStorage>>,
pub key:Option<String>,
pub value:Option<String>,
pub base:Option<String>



}
#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryRawLayout{
   pub storage: Vec<FoundryStorage>,
   pub types:FoundryTypeRaw,
}



