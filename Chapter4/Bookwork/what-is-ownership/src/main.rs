fn main() {
    println!("Hello, world!");

    {
        let s = String::from("Hello");

        println!("{s}");
    }

    // Uncomment this line to cause an error: s is no longer in scope
    //println!("{s}");

    // Unlike a string literal, a String can be mutated
    let mut s = String::from("hello");
    
    s.push_str(", world!");
    
    println!("{}", s);

    /*Multiple Variables and interacting with move */

    // Different types have different copying behaviours
    // Simple data types, like integers, will be copied
    let x = 5;
    let y = x; // creates a deep copy of the data
    // Possible because an int is of fixed size

    let s1 = String::from("hello");
    let s2 = s1;
    // Looks similar, but only the "String" data is copied
    // i.e the memory locations
    // When s1, s2 go out of scope, Rust calls "drop" on the data
    // But s1, s2 refer to the same data - "double free" error can occur
    // To get around this, rust considers s1 invalid after s2 = s1
    // println!("{}", s2); // Uncommenting this line will cause an error

    // Rust will NEVER automatically create a deep copy
}
