// Assignment 2

// Q # 4. Write a program to know the result of the
// test (Pass/Fail) by using a user defined
// function, and perform the following operations:
// (Note: Consider marks of only 2 subjects for
// the sake of simplicity. Maximum marks are 100 for each subject.)
    // a. Add marks and print the total.
    // b. Calculate the percentage and print it,
        //  return percentage to main function
    // c. If percentage >= 70, Print Pass, Else, print Fail.

fn main() {
    // percent & T/F -> return pass or false
    let(percent, pass) = percentage(90.0, 90.0);

    // Print percentage
    println!("Percentage: {}", percent);
    
    // Print pass\fails
    isPass(pass);
}

fn percentage(ics:f64, ict:f64) -> (f64, bool)
{
    // Return percentage 
    let sum:f64 = ics + ict;                // Summation of maerks of subjects
    let percent:f64 = sum/200.0 * 100.0;    // percentage
    // T -> % >= 70.0, F -> % < 70.0
    let pass:bool = {
        if percent >= 70.0{
            true
        }
        else{
            false
        }
    };

    (percent, pass)
}

fn isPass(pass:bool){
    //Chech percentage is above 70

    // Percentage >= 70.0
    if pass{
        println!("Congratulations! You passed it.");
    }

    // percentage < 70.0
    else{ 
        println!("Sorry! You failed it.");
    }
}