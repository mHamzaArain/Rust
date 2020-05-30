// Thread example

use std::thread;
use std::time::Duration;

fn main() {
    get_two_sites();
}

fn get_two_sites() {
    //Spawn teo threads to do work.
    let thread_1 = thread::spawn(|| {
        // thread::sleep(Duration::from_millis(500)); // When this work, the thread delays thread_1 hence thread_2 worked 
        println!("Thread One")
    });

    let thread_2 = thread::spawn(|| {
        println!("Thread 2")
    });

    // Wait both threads to complete.
    thread_1.join().expect("Thread one panicked!");
    thread_2.join().expect("Thread two panicked!");
}

