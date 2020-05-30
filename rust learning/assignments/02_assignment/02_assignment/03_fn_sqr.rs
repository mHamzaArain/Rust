// Assignment 2

// Q # 3. Write a rust program and define a user
//  defined function that receives a number and
// return the number itself and its square by using tuple.

fn main() {
    //Calling function & return number with its square
    let (n, _sqr) = square(3.0);
    println!("Square of {} is {}.println!",n, _sqr);
}

fn square(num: f32) -> (f32, f32){
    // Return number & its square
    let sqr = num * num;
    (num, sqr)
}