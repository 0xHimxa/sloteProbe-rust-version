
use clap::{Parser};









//what this conflicts_with_all does is it prevent 
//u can't use binary with json,yaml,xml
//only one can be passed at a time



// struct Cli {
//     #[arg(short, long)]
//     json: bool,
    
//     #[arg(short, long)]
//     yaml: bool,
    
//     #[arg(short, long)]
//     xml: bool,
    
//     // This conflicts with all other formats
//     #[arg(short, long, conflicts_with_all = &["json", "yaml", "xml"])]
//     binary: bool,
// }










//requiring some fields if related one is provided


#[derive(Parser)]

struct Cli {

  #[arg(short,long)]
  output:bool,



  //out put file is required if output is true
  #[arg(short='f',long,required_if_eq("output" ,"true"))]
  output_file: Option<String>,





//accepting more than one input for one fieild
    // Accept one or more files
    #[arg(num_args = 1..)]
    files: Vec<String>,
    
    // Accept exactly two values
    #[arg(short, long, num_args = 2)]
    point: Vec<f64>,
    
    // Accept 0 to 3 values
    #[arg(short, long, num_args = 0..=3)]
    options: Vec<String>,
    
//adding custom helps for feild

       #[arg(
        short, 
        long, 
        help = "Output format",
        long_help = "Specify the output format. Available options: json, yaml, toml"
    )]
    format: Option<String>,
}




















fn main(){
let cli = Cli::parse();

if cli.output {

    println!("output is true and output file is {:?}",cli.output_file);
}else{

    println!("output is false");
}

}