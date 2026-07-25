

use tokio::task::JoinSet;

#[tokio::main]

async fn main(){

  
//Spawn a task- like creating a promist but it start immediately

 let handle1 = tokio::spawn(async{
  println!("Task 1 started");
 tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
 println!("Task 1 finished");
 return 200;
 });
 
 let handle2 = tokio::spawn(async{
  println!("Task 2 started");
 tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
 println!("Task 2 finished");
 return 100;
 });


   // Await the results - like Promise.all
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();
    println!("Results: {} and {}", result1, result2);


    joinset_more_optimiseze().await;

}





async fn joinset_more_optimiseze() {
  //it kinda similar to promise.all, but we are passing the task in
  //some of it diffrencet with it is that, it return the first task that was
  //completed first, just is it those not preserve order


  //if our fn return early due to some stuffs, it will abort and stop the other
  //task from runing in background

    let mut join_set = JoinSet::new();
    
    for i in 0..5 {
        join_set.spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100 * i)).await;
            println!("Task {} done", i);
            i * 2
        });
    }
    
    // Wait for all tasks to complete
    while let Some(result) = join_set.join_next().await {
        println!("Got result: {}", result.unwrap());
    }
}



// this is more similar to promise.all, but it is less optimizeze 
// it wait for all the task to complete, even if the first task is completed
// and even if the first task is completed first, it will wait for all the task to complete

//if we return ealy from the function and the task are not done, it will be ruining them in backgroud
//it will not abort like aabove did


// async fn join_all_less_optimized() {
//     let tasks: Vec<_> = (0..5).map(|i| {
//         tokio::spawn(async move {
//             tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//             i * 10
//         })
//     }).collect();
    
//     // Wait for all tasks - like Promise.all
//     let results = join_all(tasks).await;
//     for result in results {
//         println!("Task result: {}", result.unwrap());
//     }
// }