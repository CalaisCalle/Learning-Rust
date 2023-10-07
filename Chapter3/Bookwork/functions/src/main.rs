fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(5);
    print_labeled_measurement(5, 'h');

    // Statements do not return a value
    let _ = 6; // This is a statement
    // let x = let y = 6; (this will cause an error)

    let y = {
        let x = 1;
        x + 1
    };

    println!("Y is {y}.");

    let f = five();
    println!("five: {f}");

    let s = plus_one(f);
    println!("five plus one: {s}");
}

fn another_function() {
    println!("Another function.");
}

fn parameter_function(x: u32) {
    println!("The value of x is {x}!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with a return type/value.
fn five() -> i32 {
    5 // This is the last expression, so is the value returned
}

// Function with a return type and parameters

fn plus_one(x: i32) -> i32 {
    x + 1 // Add a semicolon to produce an error.
}