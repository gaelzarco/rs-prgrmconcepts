// VARIBALES AND MUTABILITY

fn main() {
    // LET & LET MUT
    // vars are immutable by default
    // let x: i32 = 5;
    // println!("The value of x is: {x}");
   //  x = 6; error: cannot assign twice to immutable variable `x`

    // let mut y: i32 = 10;
    // println!("The value of y is: {y}");
    // y = 11;
    // println!("The value of y is: {}", y);

    // CONSTANTS
    // cannot use mut with constants, must annotate type, name is uppercase, use underscores
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    // Shadowing is different from using mut because it creates a new var rather than mutating the original var
    // it also enables us to change the vars type which is not possible with mut

    // we first bind b to a value of 5, then we can rebind it to a value of 5 + 1 after the first println!
    let b = 5;
    println!("The value of first-b is: {}", b);

    // new var b shadows previous b var
    let b = b + 1;

    {
        // new var b shadows previous b var but only until this inner scope ends
        let b = 5 * 2;
        println!("The value of b in the inner scope is: {}", b);
    }

    // b returns to the value of the shadowed in this outer scope b
    println!("The value of shadowed-b is: {}", b);
}
