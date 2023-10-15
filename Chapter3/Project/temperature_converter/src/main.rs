use std::io;

fn main() {
    let mut user_input = String::new();

    let f: f64 = loop {
        println!("Enter a temperature in Fahrenheit");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input!");

        // Make sure the user input is valid from the user.
        let f: f64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // −459.67 F is the lowest temp possible
        if f < (-459.67) {
            println!("Must be greater than −459.67!");
            continue;
        }
        break f;
    };

    let c: f64 = (f - 32.0) * 5.0 / 9.0;

    println!("In Celsius, that's {c} degrees!");
}
