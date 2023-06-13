// CONTROL FLOW

fn main() {
    println!("the good stuff");

    if_statements(None);
}

fn if_statements(variable: Option<i32>) {
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    // The types of both arms must be the same or else the code will not compile
    let number: i32 = if variable.is_some() { variable.unwrap() } else { 3 };

    // blocks of code associated with the conditions in if expressions are called arms
    // if you do not provide an else statement and the condition is false,
    // the program will just skip the if block and move to the next bit of code.
    // conditions must be of type bool or the program will not compile/
    // must be explicit and always provide if with a Boolean as its condition.
    if number > 5 {
        print!("condition was true");
    } else if number == 3 {
        println!("number is {number}. just right...")
    } else {
        print!("condition was false");
    }
}