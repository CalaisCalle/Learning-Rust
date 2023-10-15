use std::io;

fn main() {
    // Using u64 for the higher possible values
    let fib: u64 = loop {
        let mut user_input = String::new();

        println!("How many iterations?");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input!");

        // Make sure the user input is valid from the user.
        let n: u64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error!");
                continue;
            }
        };

        // After some experimentation, anything bigger than 93 causes an overflow
        if n > 93 {
            println!("The {n}th fibonnacci number is too large to be represented by 64 bits");
            println!("There are ways of dealing with this, but I don't know them.");
            continue;
        }

        break n;
    };

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    // Add the = sign so that it's inclusive
    // Assuming 0, 1, and the first and second fib numbers...
    for _ in 2..=fib {
        let c: u64 = a + b;
        a = b;
        b = c;
    }

    println!("Fib number: {b}");
}
