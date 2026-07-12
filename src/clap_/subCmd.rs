



use std::path::PathBuf;

use clap::{Parser, Subcommand};

//SubCommands

///are like diffence modes or action out tool can do

#[derive(Parser)]
//1. main cli struct
struct Cli{

// we  can make a command global by specify it 



   // This is a GLOBAL argument - available to all subcommands
    #[arg(short, long, global = true)]
    verbose: bool,

    //this tells clap: "the subcommand goes here"
    #[command(subcommand)]
    command: Commands,
}



//2. Difine the subcommands as an enum

#[derive(Subcommand)]
//subCommands can also have there own fields 
enum Commands{
    //each variant is a subcommand
    Create {
        #[arg(short, long)]
        name: String,
        
    },
    Delete {
        #[arg(short, long)]
        name: String,
        
    },
    List,
    Update {
        #[arg(short, long)]
        name: String,
        
    },
    

}
    







//we can also have subcommend inside another sub command
//git commoand for example

#[derive(Subcommand)]
enum GitCommands {
    /// Manage git remotes
    Remote {
        #[command(subcommand)]
        action: RemoteCommands,
    },
    
    /// Commit changes
    Commit {
        #[arg(short, long)]
        message: String,
    },
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Add a remote repository
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        url: String,
    },
    
    /// Remove a remote repository
    Remove {
        #[arg(short, long)]
        name: String,
    },
    
    /// List all remotes
    List,
}













fn main(){

let cli = Cli::parse();

  // Step 3: Match on which subcommand was used
    match cli.command {
        Commands::Create{name} => {
            println!("{} Creating a new file...", name);
        }
        Commands::Delete{name} => {
            println!("{} Deleting a file...", name);
        }
        Commands::List => {
            println!("Listing all files...");
        }
        Commands::Update{name} => {
            println!("{} Updating a file...", name);
        }
    }
}



