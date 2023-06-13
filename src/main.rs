fn main() {
    another_function(12)
}

fn another_function(x: i32) {
    // statement (does not return a value)
    let int: i32 = x;

    // let y = 6 does not return a value so there isnt anything for x to bind to
    // this results in an error
    // let x = (let y = 6);

    // Calling a function/macro is an expression. 
    // A new scope block created with curly brackets is an expression.

    let y: i32 = {
        let b: i32 = 3;
        // no semicolon at the end of the expression
        // if you add a semicolon to the end of an expression, it becomes a statement
        // and will not return a value
        b + 1
    };
    // The piece of code above is an expression that evaluates to 4

    println!("the number is {int}");
    print!("the value of y is {y}");
}

// statements are instruction that perform an action and do not return a value
// expressions evaluate to a resultant value