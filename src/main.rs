mod core;

 

use core::artifact_parser::foundry::ArtifactFile;




fn main(){

  
  let artifact_file=ArtifactFile::open("t2.json").unwrap();
  let foundry_raw_layout=artifact_file.load_foundry_artifact().unwrap();
  let normalized=ArtifactFile::normalize_artifacts(&foundry_raw_layout).unwrap();
  println!("{:#?}",normalized);


  




}