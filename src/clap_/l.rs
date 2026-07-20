

use std::thread;
fn main(){

    //closure
//closure are similar to functions but they are anonymous
//we can specify the type of param it accept or not 
//we can return the value from the closure similar to fn 
// if we did not specify or notate the type of it param it will be defaulted 
// to the first param we gave it
let add = |a:i32,b:i32| a+b;
println!("{}",add(1,2));


let t_x = |x| x;
println!("{}",t_x(String::from("hello")));
//this will cause error due to it now have string as it notation

//t_x(4);
    


// here are some of ways we can delare a closure;
//if they accept multiple input param we need to explicitly give the type
// anotetion  or // the first time we call it elemt will be assign as it param


let add_one = |a:i32|->i32 {a+1};
let add_one_v2 = |a:i32| {a+1};

//though this type will not compile till we call it with a values
// which the value type will now be assing to it 
//like we did for above example in t_x

//let add_one_v3 = |x| x+1;


let f_our = add_one(1);
let sume  = add_one(f_our);
println!("{}",sume);



//closures can mutable brrow values from the environment
//closures can borrow immutable reffreceen values from the environment
//closures can move values from the environment

// similar to what a fn can do as well
//though it will be determined  based on what the closure is doning
//with the variable in it body



let mut  list = vec![1, 2, 3];
println!("Before defining closure: {list:?}");



//here is a closure that captures an immutable reffrence from the list
//note we did not use any ref sign or & sign 
// its automaticly inferred that we are borrowing since that is 
//what println required to be able to print the list
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    // note clousre are been called as if they where a fn
    only_borrows();
    println!("After calling closure: {list:?}");

    //the imutable borrow end here since we did not longer call the closure
   // only_borrows();






// here is a closure that captures an mutable borrow from the list
//note we did not use any ref sign or & sign 
// its automaticly inferred that we are borrowing mutable since that is 
//what println required to be able to print the list
let mut mut_borrows = || {
    list.push(4);
    println!("From closure: {list:?}");
};
//note they can not be  print or  anyvalue that use ref of list
//in between since that will cause a error of immutable broowor and mutable
//broowor at the same time

mut_borrows();
println!("After calling closure: {list:?}");






// moving  a variable to a move closure aka taking complete  owner ship
//we need to specify by adding the move keyword
//we can move complex type like string,vec
//like we did below

// let list = vec![1, 2, 3];
//     println!("Before defining closure: {list:?}");

//     thread::spawn(move || println!("From thread: {list:?}"))
//         .join()
//         .unwrap();

 }





//trait can be type generic and have specific input and outut type



//trait 