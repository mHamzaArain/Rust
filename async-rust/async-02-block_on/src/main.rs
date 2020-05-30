// Async program

use std::time::Duration;
use std::thread;

// future means task
// future::executor -> ensure execution of tasks
use futures::executor::block_on;  
// use futures::future::join

async fn download_async1(text: &str){
    thread::sleep(Duration::from_millis(2000)); // Uncomment
    println!("{:?}", text);
}

async fn download_async2(text: &str){
    println!("{:?}", text);
}

async fn get_two_sites_async(){
    let future_one = download_async1("https://www.foo.com");
    let future_two = download_async2("https://www.bar.com");

    // future::join! -> macro is responsible to complete tasks from left to right then more code will run.
    futures::join!(future_one, future_two);
}


fn main() {
    // block_on -> ensure the program halt untill async function is to be done.
    block_on(get_two_sites_async());
}
