

fn print_type<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}


//a simple trait with no bound type or return type , the imlementator can return
//anything form it method or also not return anything
trait Speak {
    fn speak(&self);
}


// This trait can only work with String!
// the implementator must take string as input and return string as output
trait StringTransformer {
    fn transform(&self, input: String) -> String;
}




// One trait that works with ANY type!
//it take a type T as input and return T as output
trait Transformer<T> {
    fn transform(&self, input: T) -> T;
}

// Now implement for different types
struct Reverser;
impl Transformer<String> for Reverser {
    fn transform(&self, input: String) -> String {
        input.chars().rev().collect()
    }
}

struct Doubler;
impl Transformer<i32> for Doubler {
    fn transform(&self, input: i32) -> i32 {
        input * 2
    }
}

struct ToString;
impl Transformer<f64> for ToString {
    fn transform(&self, input: f64) -> f64 {
        input // Just returns the same, but you get the idea!
    }
}



// Implement for different types
struct Dog;
struct Cat;
struct Robot;

impl Speak for Dog {
    fn speak(&self)  {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

impl Speak for Robot {
    fn speak(&self) {
        println!("Beep boop!");
    }
}






trait Processor<T> {
    fn process(&self, input: T) -> T;
}

struct Adder;
impl Processor<i32> for Adder {
    fn process(&self, input: i32) -> i32 {
        input + 1
    }
}

impl Processor<String> for Adder {  // ERROR!
    fn process(&self, input: String) -> String {
        format!("{}!", input)
    }
}






// Input type: T, Output type: U
trait Convertable<T, U> {
    fn transform(&self, input: T) -> U;
}

// Convert String -> i32
struct StringToInt;
impl Convertable<String, i32> for StringToInt {
    fn transform(&self, input: String) -> i32 {
        input.parse().unwrap_or(0)
    }
}










trait Mapper<I, O> {
    fn map(&self, input: I) -> O;
}

struct SumVec;
impl Mapper<Vec<i32>, i32> for SumVec {
    fn map(&self, input: Vec<i32>) -> i32 {
        input.iter().sum()
    }
}

struct StringLength;
impl Mapper<String, usize> for StringLength {
    fn map(&self, input: String) -> usize {
        input.len()
    }
}

struct IsEven;
impl Mapper<i32, bool> for IsEven {
    fn map(&self, input: i32) -> bool {
        input % 2 == 0
    }
}





//both of this do same work


// fn process<I, O, M: Mapper<I, O>>(mapper: M, input: I) -> O {
//     mapper.map(input)
// }

// Or with where clause (cleaner for multiple bounds):
fn process<I, O, M>(mapper: M, input: I) -> O 
where 
    M: Mapper<I, O>
{
    mapper.map(input)
}








//now we enter asscociated type of trait 
// this type of trait will only work for single type 
// we can not use it for same type for i32 and String and again for  i32  like we did above


//eg if been done for a struct it cant be done for it again till now diffrence struct



trait Container {
    //Associated Type
    //Unlike generic type parameters (like T, U), an associated type is a placeholder
    //for a type that is defined *inside* the trait implementation.
    //This means a struct can only implement a specific Container for ONE specific type.
    type Item;
    
    fn insert(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

// Implement for Vec<i32>
struct IntBox {
    data: Vec<i32>
}

impl Container for IntBox {
    // Here we fix the associated type "Item" to "i32" for this implementation
    type Item = i32;
    
    fn insert(&mut self, item: i32) {
        self.data.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}





trait Collection{
type Input;
type Output;

fn process(&self,input:Self::Input)->Self::Output;



}



struct MyCollection{
    data:Vec<i32>
}

impl Collection for MyCollection{
    //here we lockin the tyep for it
    type Input = i32;
    type Output = i32;

    fn process(&self,input:i32)->i32{
        input+1
    }
}






fn main() {
    print_type(5);
    print_type("hello");
    print_type(vec![1, 2]);
    
    let dog = Dog;
    let cat = Cat;
    let robot = Robot;
    
    dog.speak();   // "Woof!"
    cat.speak();   // "Meow!"
    robot.speak(); // "Beep boop!"


    // usage of trait bound type 
    let reverser = Reverser;
    println!("{}", reverser.transform(String::from("hello")));

    let doubler = Doubler;
    println!("{}", doubler.transform(5));

    let to_string = ToString;
    println!("{}", to_string.transform(5.5));



    // usage of processor trait
    let adder = Adder;
    println!("{}", adder.process(5));
    println!("{}", adder.process("hello".to_string()));


    //Mapper trait


  let sum_vec = SumVec;
  println!("{}",process(sum_vec,vec![1, 2, 3]));
    

}

