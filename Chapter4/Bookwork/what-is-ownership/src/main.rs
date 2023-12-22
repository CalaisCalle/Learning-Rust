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

    // This looks superficially similar to a shallow copy in other languages
    // The capacity, pointer, and length are copied without copying the data in memory
    // However, the invalidation of the first variable makes this a *move* operation

    // Rust will NEVER automatically create a deep copy

    /* Variables and data interacting with clone() */

    let s3 = s2.clone();
    println!("{s2}, {s3}"); // s2 is still valid after the clone


    /* Stack-Only Data: Copy */
    // The following is valid code because integers have known, fixed size at compile time
    let x = 5;
    let y = x;
  
    println!("{x}, {y}");

    /* Ownership and Functions */

    takes_ownership(s3); 
    // s3 no longer valid
    // Uncommenting the following line causes a compile time error
    // println!("{s3}");

    makes_copy(x);



    /* Return Values and Scope */
    let s4 = gives_ownership();

    let s5 = takes_and_gives_back(s2);

}

/* OwnerShip and Functions */

fn takes_ownership(some_string: String) {
    println!("{some_string}");
    // Here, some string goes out of scope and drop is called.
    // Memory is freed.
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Integer goes out of scope, Nothing special happens.



/* Return Values and Scope */
fn gives_ownership() -> String {// gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
        // moves out to the calling
        // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

a_string  // a_string is returned and moves out to the calling function
}
