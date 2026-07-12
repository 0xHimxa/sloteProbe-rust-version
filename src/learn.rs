
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     let serialized = serde_json::to_string(&point).unwrap();
//     println!("serialized = {}", serialized);

//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//     println!("deserialized = {:?}", deserialized);
// }

use std::f64::NAN;













#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Customer {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Order {
    customer: Customer,
    items: Vec<String>,
    total: f64,
}

#[derive(Debug, Default,Clone,PartialEq)]
struct Dashboard {
    orders: Vec<Order>,
   // last_update: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, PartialEq,Eq,Hash)]
  struct Data {
    id: i32,
    //score: f64,
    name: String,
}

#[derive(PartialEq,PartialOrd, Hash)]
struct Points {
    x: i32,
    y: i32,
   // score: f64,
}






use clap::Parser;

/// Simple calculator program
#[derive(Parser)]
#[command(name = "calc")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Does arithmetic operations", long_about = None)]
struct Cli {
    /// First number to use
    first: i32,
    
    /// Second number to use
    second: i32,
    
    /// Operation to perform (add, subtract, multiply, divide)
    #[arg(short, long, default_value_t = String::from("add"))]
    operation: String,
}









fn main() {


// let p = Data {
//     id : 1,
//     score : 10.9,
//     name : "John".to_string(),
// };

// let p2 = Data {
//     id : 1,
//     score : 10.9,
//     name : "John".to_string(),
// };


// println!("{}", p == p2);


// let customer = Customer{
//     id : 1,
//     name : "John".to_string(),
//     email : "[EMAIL_ADDRESS]".to_string(),
// };


// let customer2 = Customer{
//     id : 2,
//     name : "John".to_string(),
//     email : "[EMAIL_ADDRESS]".to_string(),
// };




// println!("{}", customer == customer2);


// let orders: Vec<Order> = vec![Order{
//     customer : customer2,
//     items : vec!["apple".to_string(), "banana".to_string()],
//     total : 10.9,
// }];


// let mut dashboard = Dashboard{
//     orders : orders,
//   //  last_update : Some(chrono::Utc::now()),
// };




// let mut dashboard2 = dashboard.clone();




// //println!("{}", dashboard2 == dashboard2);


// println!("{:?}", dashboard);

// //Orb is the strictest comparison trait of partial comparsion trait
// #[derive(Debug,PartialEq, PartialOrd, Ord, Eq)]
// struct Score {
//     points: i32,  // i32 is Ord and hence PartialEq, PartialOrd and Ord
//     //orb dont not support floatsNumber as well due to NAN

// }

//  let s1 = Score { points: 100 };
//     let s2 = Score { points: 200 };

//    //orb provid us with .max() and min() methods 
//    let res = s1.max(s2);
//     println!("{:?}", res);
    
    // Always returns an Ordering - never None!
  //  let result = s1.cmp(&s2);
    //println!("{:?}", result);  // Always Less, Equal, or Greater


     let cli = Cli::parse();
    
    match cli.operation.as_str() {
        "add" => println!("Result: {}", cli.first + cli.second),
        "subtract" => println!("Result: {}", cli.first - cli.second),
        "multiply" => println!("Result: {}", cli.first * cli.second),
        "divide" => println!("Result: {}", cli.first / cli.second),
        _ => println!("Unknown operation"),
    }

}