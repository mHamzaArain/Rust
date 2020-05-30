// Assignment # 2

// Q # 2. Write a rust program and define
// a user defined function that receives 4
// arguments of different data types
// (integer, float, boolean, string) and print
// them on the console.

fn main() {
    // Calling function & passing 4 args
    different_dataTypes(54,     // int datatype
                49.0,           // float datatype
                "ggg",          // string
                true)           // bool
}

fn different_dataTypes(int: i32, float: f64, strr: &str, flag: bool){
    // Function contain 4 parameteres of different data 
    println!("Integer: {}\nFloat: {}\nString: {}\nBool: {}",
        int, float, strr, flag)
}