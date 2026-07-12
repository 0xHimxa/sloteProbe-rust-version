

use strum::Display;
use clap::{Parser,ValueEnum};

#[derive(Parser)]

//clap comes with some builin validators
struct Cli {

     // this validit the number and make sure it interger and fit in u8
    #[arg(short,value_parser = clap::value_parser!(u8))] //this validate the input
    id : u8,


   // This validates it's a valid path
    #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
    file: std::path::PathBuf,
    
    //This validates it's a valid URL
    #[arg(long,value_parser = clap::value_parser!(url::Url))]
    url: url::Url,






      // Percentage must be between 0 and 100
    #[arg(
        value_parser = clap::value_parser!(u8).range(0..=100)
    )]
    percentage: u8,





      #[arg(
        short,
        long,
        default_value = "info",
        value_parser = ["debug", "info", "warn", "error"]
    )]
    log_level: String,



      #[arg(short, long, default_value_t = LogLevel::Info)]
    log: LogLevel,

       #[arg(value_enum)]
    format: OutputFormat,


    #[arg(short,long,value_parser = verify_email)]
    email: String,



        #[arg(short, long)]
    verbose: bool,
    
    // Can't use quiet and verbose together
    //which is user cant set both only one
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,
    
}



//we  can aslo have our own custom error

//though it must return type of Result<T,E> 

fn verify_email(email: &str) -> Result<(), String> {
    if email.contains('@') {
        Ok(())
    } else {
        Err("Invalid email address".to_string())
    }
}





#[derive(Copy, Clone, PartialEq, Eq, ValueEnum,Display)]
enum OutputFormat {
    #[clap(name = "json")]
    Json,
    #[clap(name = "yaml")]
    Yaml,
    #[clap(name = "toml")]
    Toml,
}








// Derive ValueEnum to use with clap
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum,Display)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}











fn main(){
let cli = Cli::parse();

println!("id is = {}", cli.id);
}