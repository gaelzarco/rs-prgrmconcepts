// VARIBALES AND MUTABILITY

fn main() {
    // vars are immutable by default
    let x: i32 = 5;
    println!("The value of x is: {x}");
   //  x = 6; error: cannot assign twice to immutable variable `x`

    let mut y: i32 = 10;
    println!("The value of y is: {y}");
    y = 11;
    println!("The value of y is: {}", y);

    // constants
    // cannot use mut with constants, must annotate type, name is uppercase, use underscores
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

}
