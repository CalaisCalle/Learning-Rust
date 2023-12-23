fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // s1 can be used here as it was previously passed by reference
    println!("The length of '{}' is {}.", s1, len);

    /* Mutable references (main)*/
    // The following is valid

    let mut s2 = String::from("hello");
    change(&mut s2);

    println!("{s2}");

    // Cannot create two references to the same data at the same time
    let r1 = &mut s2; // works
    // r2 = &mut s2; // falls over at compile time

    // Braces can be used to create a new scope, allowing for two references
    // These are not simultaneous

    let mut s3 = String::from("hello, world");

    {
        let r2 = &mut s3;
    } // r2 goes out of scope, so a new reference can be created

    let r3 = &mut s3;

    // Immutable and mutable references to the same variable cannot exist simultaneously
    // if a mutable reference is created, its scope lasts until the last time it's used
    let mut s4 = String::from("hi there");
    let r4 = &s4;

    println!("{s4}");
    let r5 = &mut s4; // Now works because the reference r4 has gone out of scope
    // Moving this line before println! will cause an error.
}

fn calculate_length(s: &String) -> usize {
    s.len() 
} // calculate_length does not have ownership of the variable s
// When s goes out of scope, it is not dropped as a result.

// Trying to change the value of a borrowed variable results in an error:
// Uncommenting this function will cause a compile-time error.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

/* Mutable References (functions) */

// The following will allow changes to be made to the borrowed object
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This function attempts to create a dangling reference.
// Uncommenting it creates a compile-time error
// fn dangle() -> &String {
//     let s = String::from("hi"); // Create a string

//     &s
// } // s goes out of scope, so &s references nothing