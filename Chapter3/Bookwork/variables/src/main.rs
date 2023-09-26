// Constants can be declared in the global scope
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x in the outer scope is {x}");

    // Shadowed values can have different types:
    let spaces = "    ";
    let spaces = spaces.len();
    // Using mut with the first line will cause an error.
}
