// CONTROL FLOW

fn main() {
    // let mut count = 3;

    // for fun
    // here we are iterating over a reversed range of numbers from 1 to 4
    // the range is inclusive of the first number and exclusive of the last number
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFT OFF!!!");
    // Looping over a collection
    // We are looping over this array and incrementing the mutable varibale index
    // index is used as the arm of the while loop and breaks when the index is atleast 5
    // let a: [i32; 5] = [10, 20, 30, 40, 50];
    // for num in a {
    //     println!("The value is {num}");
    // }

    // This approach is error prone
    // 1: we could cause the program to panic if the index length is greater than the array length
    // 2: we could cause the program to panic if the index length is less than the array length
    // 3: its slow because the compiler adds runtime code to perform the conditional check
    // 3: lets try a for loop implementation
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let mut index: usize = 0;
    // while index < 5 {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }

    // conditional loops
    // while loop runs until count = 0
    // it then breaks and prints LIFTOFF!!!
    // while count != 0 {
    //     println!("{count}!");
    //     count -= 1;
    // }

    // println!("LIFTOFF!!!");

    // break and continue apply to the innermost loops
    // loop labels begin with a single quote and are followed by a label ending in a colon
    // the label can be used to exit a specific loop rather than just exiting from the current/innermost one
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
        
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // println!("End count = {count}");

    // Instantiating a variable to hold the result of a loop
    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2; // break returns a value
    //     }
    // };

    // println!("The result is {result}");    
}

// fn loops() {
//     loop {
//         println!("again!")
//     }
// }

// fn if_statements(variable: Option<i32>) {
//     // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
//     // The types of both arms must be the same or else the code will not compile
//     let number: i32 = if variable.is_some() { variable.unwrap() } else { 3 };

//     // blocks of code associated with the conditions in if expressions are called arms
//     // if you do not provide an else statement and the condition is false,
//     // the program will just skip the if block and move to the next bit of code.
//     // conditions must be of type bool or the program will not compile/
//     // must be explicit and always provide if with a Boolean as its condition.
//     if number > 5 {
//         print!("condition was true");
//     } else if number == 3 {
//         println!("number is {number}. just right...")
//     } else {
//         print!("condition was false");
//     }
// }