use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an individual variable's storage slot layout within a Foundry artifact.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryStorage {
    /// The variable name or label in the Solidity contract.
    pub label: String,
    
    /// The byte offset within the 32-byte storage slot (0–31).
    pub offset: u8,
    
    /// The starting storage slot index, represented as a decimal or hex string in raw JSON.
    pub slot: String,
    
    /// The identifier/key pointing to the type definition in the `types` map (renamed from Rust keyword `type`).
    #[serde(rename = "type")]
    pub type_of: String,
}





/// A type alias representing the raw type dictionary output by Foundry, mapping type keys to their metadata.
pub type FoundryTypeRaw = HashMap<String, FoundryTypeInfo>;





/// Contains detailed type information for contract storage variables.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryTypeInfo {
    /// The memory encoding layout (e.g., "inplace", "mapping", "dynamic_array").
    pub encoding: String,
    
    /// The size of the type in bytes, represented as a string in raw JSON.
    #[serde(rename = "numberOfBytes")]
    pub number_of_bytes: String,
    
    /// An optional human-readable label or display type name (e.g., "uint256", "address").
    pub label: Option<String>,
    
    /// Optional field containing nested struct fields if this type represents a struct.
    pub members: Option<Vec<FoundryStorage>>,
    
    /// The type key for mapping keys, present only if this type is a mapping.
    pub key: Option<String>,
    
    /// The type key for mapping values, present only if this type is a mapping.
    pub value: Option<String>,
    
    /// The underlying element type key, present if this type is an array or pointer type.
    pub base: Option<String>,
}







/// Represents the top-level `storageLayout` JSON object produced by Foundry/solc compilation outputs.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundryRawLayout {
    /// Array of storage entries defined in the contract.
    pub storage: Vec<FoundryStorage>,
    
    /// Dictionary of type definitions referenced by the `storage` items.
    pub types: FoundryTypeRaw,
}






/// A parsed and normalized representation of a storage entry, optimized for runtime usage
/// by using borrowed strings and parsed numerical types (`U256`, `u8`).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedStorage<'a> {
    /// Borrowed reference to the variable name.
    pub name: &'a str,
    
    /// Borrowed reference to the type name/identifier.
    pub type_of: &'a str,
    
    /// Borrowed reference to the storage label.
    pub label: &'a str,
    
    /// The storage slot index parsed into an Alloy `U256`.
    pub slot: U256,
    
    /// The byte offset within the storage slot (0–31).
    pub offset: u8,
    
    /// The size of the variable in bytes, parsed as a numeric byte count.
    #[serde(rename = "numberOfBytes")]
    pub number_of_bytes: u8,
}