



use std::path::PathBuf;

use clap::Parser;


#[derive(Parser)]
struct Cli{
//the short and long  means -f or --file  will both work
///though we can change that if u want by short='i' or long='output'
/// though default behavious is better
/// 
///  
 #[arg(short,long)]   
file: PathBuf,

/// we can asign default value using this
/// and when it not be provided it use 1 as default
#[arg(short,long,default_value_t = 1)]
count: u32,


//accept bool flags by just adding -e or --enabled
#[arg(short, long)]
enabled: bool,



//Option type we can pass value if not passed it will be None by default
//optional input
#[arg(short, long)]
output: Option<String>,

}

fn main(){

let cli = Cli::parse();

println!("File: {}", cli.file.display());
println!("count: {}", cli.count);
println!("enabled: {}", cli.enabled);
println!("output: {:?}", cli.output);

}
