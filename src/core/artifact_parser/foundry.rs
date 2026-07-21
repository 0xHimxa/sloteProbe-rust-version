use alloy::primitives::U256;
use std::fs::File;
use std::io::BufReader;
use super::types::{FoundryRawLayout, NormalizedStorage};

/// Wrapper around a target EVM artifact file on disk.
pub struct ArtifactFile {
    file: File,
}

impl ArtifactFile {
    /// Opens the specified file path and wraps it in an `ArtifactFile` instance.
    ///
    /// # Errors
    /// Returns standard `std::io::Error` if the file cannot be found or accessed.
    pub fn open(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        Ok(Self { file })
    }

    /// Reads and deserializes the contract artifact JSON into the raw `FoundryRawLayout` layout structure.
    ///
    /// # Errors
    /// Returns a `String` error if `serde_json` fails to parse the file or expected JSON keys are missing.
    pub fn load_foundry_artifact(&self) -> Result<FoundryRawLayout, String> {
        // Wrap the file stream in a buffered reader for optimized I/O operations
        let reader = BufReader::new(&self.file);
        
        // Parse the JSON stream directly into the raw layout struct
        let layout: FoundryRawLayout = serde_json::from_reader(reader)
            .map_err(|e| format!("invalid fields in the artifact file: {}", e))?;
            
        Ok(layout)
    }

    /// Converts raw strings in `FoundryRawLayout` into parsed numerical types (`U256`, `u8`) 
    /// and constructs borrowed `NormalizedStorage` entries.
    ///
    /// # Errors
    /// Returns a `String` error if:
    /// - A referenced variable's `type_of` identifier is missing from the `types` map.
    /// - The `slot` string cannot be parsed as a base-10 `U256`.
    /// - `number_of_bytes` cannot be parsed into a `u8`.
    pub fn normalize_artifacts(
        artifacts: &FoundryRawLayout,
    ) -> Result<Vec<NormalizedStorage<'_>>, String> {
        let normalized: Vec<NormalizedStorage> = artifacts
            .storage
            .iter()
            .map(|v| {
                // Fetch the corresponding type metadata from the `types` lookup dictionary
                let type_info = artifacts
                    .types
                    .get(&v.type_of)
                    .ok_or_else(|| format!("Type metadata missing for: {}", v.type_of))?;

                // Map raw JSON string properties into strongly-typed numerical values
                Ok(NormalizedStorage {
                    name: &v.label,
                    type_of: &v.type_of,
                    label: &v.label,
                    // Convert decimal slot string into Alloy's U256 representation
                    slot: U256::from_str_radix(&v.slot, 10)
                        .map_err(|e| format!("Failed to parse slot value: {}", e))?,
                    offset: v.offset,
                    // Parse byte-length string into an unsigned 8-bit integer
                    number_of_bytes: type_info
                        .number_of_bytes
                        .parse::<u8>()
                        .map_err(|e| format!("Invalid number of bytes: {}", e))?,
                })
            })
            // Collect the iterator results; short-circuits on the first `Err` encountered
            .collect::<Result<Vec<_>, String>>()?;

        Ok(normalized)
    }
}