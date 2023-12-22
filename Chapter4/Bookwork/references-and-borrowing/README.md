# 4.2 References and Borrowing

Previously, using tuples to return borrowed data from a function was shown as one solution to the problem of borrowed data dropping out of a function scope:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

In the above code, the `String` `s1` was moved into `calculate_length`. 
When the `String` `s` in gets to the end of the scope in `calculate_length`, it gets dropped, but `main()` uses the string after the call to `calculate_length`.
So, `calculate_length` must also return the string.

Instead, `calculate_length` could be reformatted to use a reference instead, and `main` could provide a reference to a `String`.
A *reference* is an address that can be followed to access the data stored at said address; that data is owned by some other variable.
This is similar to a pointer, except a refence is always guaranteed to point to a valid value of a particle type for the lifetime of that reference.

A second version of `calculate_length` that uses a reference:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The tuple code in the variable declaration and function return value is gone, and `s1` is passed into `calculate_length` as `&s1`.
A variable name prefixed with an ampersand means "a reference to this variable".
The `&s1` syntax creates a reference that referes to the value of `s1`, but does not own it.
In the definition of `calculate_length`, the function signature indicates `&String` to indicate that it takes a reference to a `String`, rather than a `String`.

The scope in which variable `s` is valid is the same as any function parameter's scope. 
However, `s` is not dropped when `s` stops being used, as `s` does not have ownership of the data.
When functions use references as parameters, they don't need to return those values in order to give back ownership: they never had it.

The action of creating a reference to a variable is called *Borrowing*.

Trying to modify the value of a borrowed variable will result in a compile time error:

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```


```
$ cargo run
   Compiling references-and-borrowing v0.1.0 (..\references-and-borrowing)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
  --> src\main.rs:18:5
   |
18 |     some_string.push_str(", world");
   |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
17 | fn change(some_string: &mut String) {
   |                         +++
   For more information about this error, try `rustc --explain E0596`.
error: could not compile `references-and-borrowing` (bin "references-and-borrowing") due to previous error
```

References, like variables, are immutable by default.

## Mutable References