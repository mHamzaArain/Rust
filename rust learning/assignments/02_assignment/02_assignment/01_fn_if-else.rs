////// Assignment # 2

// Q # 1. Write a rust program and define a user defined function that
//  receives one argument of any suitable data type and print whether 
// the number is positive, negative or equal to zero.
// (hint: if/else)

fn main() {
    // Calling function
    check_no(4);    // Positive 
    check_no(-4);   // Negative
    check_no(0);    // Zero
}

fn check_no(number: i32){
    //Check whether no is positive, negative or zero
    if number > 0{       // Positive number
        println!("{} is positive", x)
    }
    else if number < 0{  // Negative number
        println!("{} is negative", x)
    }
    else{           // Zero number
        println!("{} is zero", x)
    }
}