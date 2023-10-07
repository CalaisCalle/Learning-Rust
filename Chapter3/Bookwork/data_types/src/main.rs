fn main() {
    // parse can return different data types
    // variable assignment must be annotated.
    let guess: u32 = "42".parse().expect("Not a number!");

    // Floating point types
    let x = 2.0; // f64, no annotation as f64 is default

    let y: f32 = 3.0; // f32, must be annotated.

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; // Returns 3

    // Booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Char Character
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // #######
    // Compound types
    // #######

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    // Accessing by index:
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    // Arrays:
    let a = [1, 2, 3, 4, 5]; // Declare an array

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Declare array with explicit type and size

    let a = [3; 5]; // Declare an array with the same value for each element

    // Indexing
    let first = a[0];
    let second = a[1];

    // Invalid Indexing
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
