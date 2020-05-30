// TYPES OF ERRORS :
// 1. Syntax errors - Syntax errors represent grammar errors in the use of the programming language
// 2. Runtime errors - Runtime errors occur when a program with no syntax errors asks the computer to do something that the computer is unable to reliably do.
// 3. Logic errors - Logic errors occur when there is a design flaw in your program.


// // 1. ////////////USING A PANIC! BACKTRACE
// // Debug: RUST_BACKTRACE=1 cargo run
// // PANIC! MACRO :
// //    Rust has the panic! macro. When the panic!
// //    macro executes, your program will print a failure
// //    message, unwind and clean up the stack, and then
// //    quit. This most commonly occurs when a bug of 
// //    some kind has been detected and it’s not clear
// //    to the programmer how to handle the error.

// fn main() {
//     let v = vec![1, 2, 3];
    
//     // panic macro to debug after this line
//     panic!();
//     v[99];
// }

// //  2. Custom error
// #[allow(unused_mut)] //A lint attribute used to suppress the warning; username variable does not need to be mutable
// fn main() {
//     let mut username = String::new();

//     // some code to get the name
//     if username.is_empty() {
//         panic!("Username is empty!");
//     }
//     println!("{}", username);
// // -------------- Compile-time error --------------
// // thread 'main' panicked at 'Username is empty!', src/main.rs:8:9
// }

// // 3. Recovereable Error
// use std::fs::File;  // crate to approach file

// fn main() {
//     let f = File::open("exists.txt");   // Exists
//     // let f = File::open("not-exist.txt"); // not exists
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("There was a problem opening the file: {:?}", error)
//         },
//     };
// }

// // 4. MATCHING ON DIFFERENT ERRORS
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("exists.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
//             },
//             other_error => panic!("There was a problem opening the file: {:?}", other_error),
//         },
//     };
// }

// 5. Different error in better way
// use std::fs::File;
// use std::io::ErrorKind;
// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| //Error raised: if > Create file, else > File not opened
//         {
//             if error.kind() == ErrorKind::NotFound // File not found: Create file
//             {
//                 File::create("hello.txt").unwrap_or_else(|error| // Error: Unableto create file
//                 {
//                     panic!("Tried to create file but there was a problem: {:?}", error);
//                 })
//             } 
//             else  // Error: file couldnt load
//             {
//                 panic!("There was a problem opening the file: {:?}", error);
//             }
//         });
// }

// // 6. unwrap keyword -> shortcut of matchkeyword
// // Unwrape keyword:
// //      If the Result value is the Ok variant, unwrap will return the value inside the Ok.
// //      If the Result is the Err variant, unwrap will call the panic! macro for us. 
// use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt").unwrap();
// }

// // 7. expect keyword
// // expect keyword:
// //      Another method, expect, which is similar to unwrap, lets us also choose the panic! error
// //      message. Using expect instead of unwrap and providing good error messages can
// //      convey your intent and make tracking down the source of a panic easier.
// use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// // 8. PROPAGATING ERRORS
// //      When you’re writing a function whose implementation calls something that
// //      might fail, instead of handling the error within this function, you can
// //      return the error to the calling code so that it can decide what to do. 
// use std::io;
// use std::io::Read;
// use std::fs::File;
// fn main(){
//     fn read_username_from_file() -> Result<String, io::Error>
//     {
//         let f = File::open("hello.txt");
//             let mut f = match f {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };

//         let mut s = String::new();

//         match f.read_to_string(&mut s) {
//             Ok(_) => Ok(s),
//             Err(e) => Err(e),
//         }
//     }

//     println!("{:#?}", read_username_from_file());
// }

// // 9. ? OPERATORS
// use std::io::{self, Read};
// use std::fs::File;

// fn main(){
//     fn read_username_from_file() -> Result<String, io::Error>{
//         let mut s = String::new();
//         let mut f = File::open("hello.txt")?.read_to_string(&mut s);
//         Ok(s)
//     }
//     println!("{:#?}", read_username_from_file());

//     let a = String::from("")
// }

// //10. To panic! or not to panic!

// // struct IpAdder{
// //     V4'
// //     V6
// // }

// // this crate indicate above enum
// use std::net::IpAddr;

// fn main(){
//     let home:IpAddr = "191.168.0.1".parse().unwrap();
//                             // Ok(), Err()
//     assert_eq!(home.is_ipv4(), true);
// }

// // 11. Creating Custom Types for Validation
// // ///IN lib.rs
// pub mod cal{
//     fn positive(x: u32){
//         println!("The number is poaitive");
//     }
// }

// // ///IN main.rs
// use error_handling::cal;
// fn main(){
//     cal::positive(7);     // perfect
//     cal::positive(-7);    // err
// }

// Guesssing game with custom validation
#![allow(unused_variables)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // Generate random numbers
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop{
        // user input 
        let mut guess = String::new();
            io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let num = Guess::new(guess);
        let guess   = num.value();

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("win");
                break;
            }
        
            Ordering::Greater => println!("Too high"),
            Ordering::Less => println!("Too low"),
        }
    }
}






















