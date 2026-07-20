//serialize trait allow us to convert our value in any format like JSON, XML, etc
//while deserialize allow us to convert  from any format into our value value

//eg struct and enums



#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u8,
}





// now we use #[serde(rename = "fullName")] to rename the key in json
// this is useful when the json key is different from the rust struct key
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(rename = "fullName")]
    name: String,
    
    #[serde(rename = "ageInYears")]
    age: u8,
}




//Now thier is way to change the output of the json case
//rename_all = "snake_case" / "camelCase" / "SCREAMING_SNAKE_CASE" (Rename ALL fields)





// this struct is the same as previous one but instead of using rename
//for each field we use rename_all to change the case of all fields
#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
struct Product2 {
    product_id: u32,      // → productId
    product_name: String, // → productName
    is_available: bool,   // → isAvailable
}


//the default case for a field is that if the field is not present in the json
//the default value of that field is used
// if we out the default value at the top of the strcut it means all
//it values will be default

#[derive(Serialize, Deserialize, Debug)]
#[derive(Default)]
#[serde(default)]
struct Settings {
    volume: u8,      // Default: 0
    brightness: u8,  // Default: 0
    mode: String,    // Default: ""
}




//we can also pass in our own coustom value 
//by passing a function name to the default attribute








fn default_age() -> u8 { 18 }
fn default_country() -> String { "Unknown".to_string() }

#[derive(Deserialize, Serialize, Debug)]
struct User2 {
    #[serde(default = "default_age")]
    age: u8,
    
    #[serde(default = "default_country")]
    country: String,
    
    #[serde(default)]  // Still uses Default trait
    active: bool,
}





fn main()->Result<(),serde_json::Error>{
  // Serialize: Rust → JSON
//Create the user struct
let user = User {
    id: 1,
    name: String::from("himxa"),
    email: String::from("himxa@[EMAIL_ADDRESS]"),
    age: 21,
};



let json_string = serde_json::to_string(&user)?;
println!("{}",json_string);

//Deserialize: JSON → Rust

 let json_data = r#"{"id":2,"name":"Bob","email":"bob@example.com","age":25}"#;
 let parsed: User = serde_json::from_str(json_data)?;
println!("{:?}",parsed);



//the serge ranme will remane the key position in the json
//but when we pass in the json back it will change it to name back


   let person = Person {
        name: "Charlie".to_string(),
        age: 42,
    };
    
    let json = serde_json::to_string(&person)?;
    println!("{}", json);
    // Output: {"fullName":"Charlie","ageInYears":42}
    
    // Deserialize works with the renamed fields too
    let json_data = r#"{"fullName":"Diana","ageInYears":35}"#;
    let parsed: Person = serde_json::from_str(json_data)?;
    println!("{:?}", parsed); // Person { name: "Diana", age: 35 }




    //default testing

    let settings = Settings::default();
    let json = serde_json::to_string(&settings)?;
    println!("{:?}", json);



  let json = r#"{"country": "Canada"}"#;
  let parsed: User2 = serde_json::from_str(json)?;
  println!("{:?}", parsed);

Ok(())



}