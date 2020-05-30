//---------------------- Using await -------------------------


// use futures::executor::block_on;

// async fn hello_world() {
//     println!("hello, world!");
// }

// fn main() {
//     let future = hello_world(); // Nothing is printed
//     block_on(hello_world()); // `future` is run and "hello, world!" is printed
// }



///////////////////////////////////////////////////////////////////////////////////////

use std::time::Duration;
use futures::executor::block_on;
use std::thread;
use async_std::task;

fn main(){
    // block_on -> used because main function is not asynchronuous.
    // block_on -> halt program untill program not done yet. 
    block_on(world());
}

async fn world(){
    // wait -> just make program to wait for spacific time
    hello().await;
    task::sleep(Duration::from_secs(5)).await; 
    print!(" World");    
    
}

async fn hello(){      
    print!("Hello");
}