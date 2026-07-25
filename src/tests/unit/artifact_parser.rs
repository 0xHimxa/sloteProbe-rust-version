use crate::core::artifact_parser::foundry::ArtifactFile;
use crate::core::artifact_parser::types::{FoundryRawLayout, FoundryStorage, FoundryTypeInfo};
use alloy::primitives::U256;
use std::collections::HashMap;

#[test]
fn test_foundry_artifact_parsing() {
    // Open the test artifact file
    let artifact = ArtifactFile::open("t2.json")
        .expect("Failed to open t2.json artifact");

    // Load the raw Foundry layout
    let raw_layout = artifact.load_foundry_artifact()
        .expect("Failed to load foundry artifact");

    // Verify some properties of the raw layout
    assert_eq!(raw_layout.storage.len(), 18);
    assert!(raw_layout.types.contains_key("t_enum(Status)6"));

    // Normalize the storage variables
    let normalized = ArtifactFile::normalize_artifacts(&raw_layout)
        .expect("Failed to normalize artifacts");

    assert_eq!(normalized.len(), 18);

    // Verify the first variable: "status" (enum, slot 0, offset 0, 1 byte)
    let status = &normalized[0];
    assert_eq!(status.name, "status");
    assert_eq!(status.slot, U256::from(0));
    assert_eq!(status.offset, 0);
    assert_eq!(status.number_of_bytes, 1);

    // Verify the second variable: "level" (uint8, slot 0, offset 1, 1 byte)
    let level = &normalized[1];
    assert_eq!(level.name, "level");
    assert_eq!(level.slot, U256::from(0));
    assert_eq!(level.offset, 1);
    assert_eq!(level.number_of_bytes, 1);

    // Verify the owner variable: (address, slot 9, offset 0, 20 bytes)
    let owner = normalized.iter().find(|v| v.name == "owner")
        .expect("Could not find variable 'owner'");
    assert_eq!(owner.slot, U256::from(9));
    assert_eq!(owner.offset, 0);
    assert_eq!(owner.number_of_bytes, 20);
}

#[test]
fn test_open_non_existent_file() {
    let result = ArtifactFile::open("non_existent_file.json");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_normalize_missing_type_metadata() {
    // Construct a mock layout where "some_var" has type "t_missing", 
    // but "t_missing" is not in the types HashMap
    let raw_layout = FoundryRawLayout {
        storage: vec![FoundryStorage {
            label: "some_var".to_string(),
            offset: 0,
            slot: "0".to_string(),
            type_of: "t_missing".to_string(),
        }],
        types: HashMap::new(), // Empty types map
    };
    let result = ArtifactFile::normalize_artifacts(&raw_layout);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Type metadata missing for: t_missing"));
}

#[test]
fn test_normalize_invalid_slot_number_and_byted() {
    let mut types = HashMap::new();
    types.insert("t_uint256".to_string(), FoundryTypeInfo {
        encoding: "inplace".to_string(),
        number_of_bytes: "32".to_string(),
        label: Some("uint256".to_string()),
        members: None,
        key: None,
        value: None,
        base: None,
    });
    let mut types1 = HashMap::new();
    types1.insert("t_uint256".to_string(), FoundryTypeInfo {
        encoding: "inplace".to_string(),
        number_of_bytes: "256".to_string(),
        label: Some("uint256".to_string()),
        members: None,
        key: None,
        value: None,
        base: None,
    });
    let raw_layout = FoundryRawLayout {
        storage: vec![FoundryStorage {
            label: "bad_slot".to_string(),
            offset: 0,
            slot: "invalid_number_123".to_string(),
            type_of: "t_uint256".to_string(),
        }],
        types,
    };
    let raw_layout1 = FoundryRawLayout {
        storage: vec![FoundryStorage {
            label: "bad_bytes".to_string(),
            offset: 0,
            slot: "0".to_string(),
            type_of: "t_uint256".to_string(),
        }],
        types: types1,
    };
    let result = ArtifactFile::normalize_artifacts(&raw_layout);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse slot value"));

    let result2 = ArtifactFile::normalize_artifacts(&raw_layout1);
    assert!(result2.is_err());
    assert!(result2.unwrap_err().contains("Invalid number of bytes"));
}
