// DATATYPES
// rust is a statically typed language
fn main() {
    // SCALAR TYPES: represent a single calue (integers, floating-point numbers, Booleans, and characters)
    let unsigned: i32 = -10; // signed integer
    let signed: u32 = 10; // unsigned integer
    let wrap: &u32 = &signed.wrapping_add(1); // wrapping_add() is a method that checks for overflow
    println!("{}", wrap);

    // example of a compound type
    let wrap: &(u32, bool) = &signed.overflowing_add(1); // overflowing_add() is a method that checks for overflow and returns a tuple (result, bool)
    // can also use checked_add() which returns an Option<T> that is None if overflow occurs

    println!("{:#?}", wrap);

    println!("signed: {}, unsigned: {}", signed, unsigned);

    let decimal: f32 = 10.0; // floating point
    let visualseperator: u32 = 1_000_000; // visual seperator

    println!("decimal: {decimal}, visualseperator: {visualseperator}");

    let f: bool = false; // explicit bool
    let t: bool = true; // explicit bool

    println!("f: {f}, t: {t}");
    
    // chars
    // represents single Unicode scalar values
    // can represent more than ASCII (e.g. emoji)

    let char: char = '🪴'; // specify chars using single quotes as opposed to string literals
    println!("char: {}", char);

    // COMPOUND TYPES

    // Tuple type
    // Fixed length
    let balls: (char, char) = ('🪀', '🪀');
    println!("balls: {:#?}", balls);

    let tup: (i32, u32, f32) = (-500, 500, 5.0);
    println!("tup: {:?}", tup);

    let (x, y, z) = tup; // Destructuring a typle into individual variables
    println!("x: {}, y: {}, z: {}", &x, &y, &z);

    let five_point_o = tup.2; // Accessing a tuple element directly
    println!("five_point_o: {}", &five_point_o);

    let unit: () = (); // UNIT TYPE
    println!("unit: {:#?}", &unit);
    // An empty tuple used when no value is needed
    // This is the default return type of expressions that don't return anything

    // ARRAY TYPE
    // Every element of an array must have the same type
    // Arrays are fixed lengths
    // Allocated on the stack
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let b = [1; 10]; 

    println!("a: {:#?}", &a);
    println!("b: {:?}", &b);

}