

//Iterators in rust

//they allow us to loop throug collection of that without manually traking on which index is nex
// it keeps track of that for us


// in loop where we need  to track i to know which index we iteratro fix that we just loop
//through the element



// iter() -- iterate of reference to the value
// iter_mut() -- iterate of mutable refrence to the values
// into_iter() -- iterate of values it self aka take ownership



// we can acesss the value from the iterator by calling next()
//next() return Option<T> which can be Some(T) or None

// when we loop through the iterator using for loop
// it automatically calls next() in each iteration


//if we call it mannually we need to handle it and make the mutability of the elemeent  we assing it to mutable
//see below







fn main(){

 let v = vec![1,2,3];

 let mut v_iter = v.iter();


 //println!("{:#?}",v_iter.next());
 

//.map  can be collecod on an iterator and produce another iterator

// the accept and closure of the action that will be perform on each element 

   let v1: Vec<i32> = vec![1, 2, 3];
   //we need to add the .colloect to collect the modeife iterator from map
   //else the value v2 will not store the modie value instead it will contain
   //the v1 in a Map objct with the iterator
   let v2: Vec<_>  = v1.iter().map(|x| x + 1).collect();


println!("{:?}",v2);


//   .filter   can be collecod on an iterator and produce another iterator

// the filter method on iterator, filter out the elemet and return what meet the
//condition we specified

let v1: Vec<i32> = vec![1, 2, 3];

//here we filter out 2 from this list
let v2 :Vec<_> = v1.iter().filter(|x| x != 2).collect();

println!("{:?}",v2);


 




//   .zip      can be collecod on an iterator and produce another iterator
//   .enumerate   can be collecod on an iterator and produce another iterator




}