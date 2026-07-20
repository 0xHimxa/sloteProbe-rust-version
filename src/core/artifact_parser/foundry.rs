use std::fs::File;
use std::io::BufReader;
use super::types::FoundryRawLayout;
use serde::{Deserialize,Serialize};
use alloy::primitives::U256;

pub struct ArtifactFile {
    file: File,
}



#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedStorage<'a> {
    pub name: &'a str,
    pub type_of: &'a str,
    pub label: &'a str,
    pub slot: U256,
    pub offset: u8,
    #[serde(rename = "numberOfBytes")]
    pub number_of_bytes: u8,
}

impl  ArtifactFile {
   pub fn open(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        Ok(Self { file })
    }



pub fn load_foundry_artifact(&self) -> Result<FoundryRawLayout, String> {
    let reader = BufReader::new(&self.file);
    let layout: FoundryRawLayout = serde_json::from_reader(reader).map_err(|e|format!("invalid feilds in the artifact file: {}", e))?;
    Ok(layout)
   }



  pub fn normalize_artifacts(artifacts:&FoundryRawLayout)->Result<Vec<NormalizedStorage<'_>>, String>{

      
 
 let normalized: Vec<NormalizedStorage> = artifacts
            .storage
            .iter()
            .map(|v| {
                let type_info = artifacts
                    .types
                    .get(&v.type_of)
                    .ok_or_else(|| format!("Type metadata missing for: {}", v.type_of))?;

                Ok(NormalizedStorage {
                    name: &v.label,
                    type_of: &v.type_of,
                    label: &v.label,
                    slot: U256::from_str_radix(&v.slot, 10).map_err(|e|format!("Failed to parse the artifact file to FoundryRawLayout: {}", e))?,
                    offset: v.offset,
                    number_of_bytes: type_info.number_of_bytes.parse::<u8>().map_err(|e|format!("Invalid number of bytes: {}", e))?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(normalized)
  }


   

}