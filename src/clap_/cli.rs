
use clap::{Parser,Subcommand,Args,value_parser,ValueEnum};

use alloy::primitives::Address;
use std::path::PathBuf;





#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command{
    Snapshot(SnapshotArgs),
    Diff(DiffArgs),
    CheckCollision(CheckCollisionArgs),
    GenerateMigration(GenerateMigrationArgs),
}


#[derive(Args)]

struct SnapshotArgs{
//need to implement my own validators
    #[arg(long,value_parser = value_parser!(Address))]
    address: Address,

    #[arg(long)]
    chain: String,

    #[arg(long,value_parser = value_parser!(u64))]
    block: u64,

    #[arg(long,value_parser = value_parser!(PathBuf))]
    artifact: PathBuf,

    #[arg(long)]
    only: Option<String>,
    #[arg(long,value_parser = value_parser!(PathBuf), default_value = "snapshot.md")]
    out: Option<PathBuf>,
    
     #[arg(long,value_parser = value_parser!(OutputFormat), default_value = "markdown")]
    format:OutputFormat,
    #[arg(long)]
    dry_run: bool,

}

#[derive(Args)]
struct DiffArgs{
  
  #[arg(long,value_parser = value_parser!(PathBuf))]
  before:PathBuf,

  #[arg(long,value_parser = value_parser!(PathBuf))]
  after:PathBuf,

  #[arg(long,value_parser = value_parser!(PathBuf))]
  output:PathBuf,
  
   #[arg(long,value_parser = value_parser!(OutputFormat), default_value = "markdown")]
    format:OutputFormat,
  
}

#[derive(Args)]
struct CheckCollisionArgs{


  #[arg(long,value_parser = value_parser!(PathBuf))]
  before:PathBuf,

  #[arg(long,value_parser = value_parser!(PathBuf))]
  after:PathBuf,

  #[arg(long,value_parser = value_parser!(PathBuf))]
  output:PathBuf,
   #[arg(long,value_parser = value_parser!(OutputFormat), default_value = "markdown")]
    format:OutputFormat,
}

#[derive(Args)]
struct GenerateMigrationArgs{

    #[arg(long,value_parser = value_parser!(PathBuf))]
    before:PathBuf,

    #[arg(long,value_parser = value_parser!(PathBuf))]
    after:PathBuf,

    #[arg(long,value_parser = value_parser!(PathBuf))]
    out:PathBuf,
    #[arg(long,value_parser = value_parser!(OutputFormat), default_value = "markdown")]
    format:OutputFormat,
    #[arg(long)]
    verify:bool,

    #[arg(long)]
    dry_run:bool,
    
}

#[derive(Clone,ValueEnum,Debug,PartialEq,Eq,PartialOrd,Ord)]
enum OutputFormat {
    Terminal,
    Json,
    Markdown,
}


fn main(){

 let cli = Cli::parse();



}
