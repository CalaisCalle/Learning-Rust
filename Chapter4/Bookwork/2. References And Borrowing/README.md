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

The previous code can be modified to allow changes to be made to the borrowed value.
Specifically, the use of a mutable reference will allow changes:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

`s` has been changed to `mut`, and a mutable reference to the string is created (`&mut s`).
This is to make it clear that `change` will mutate the string in some way.

There is one rule to mutable references: only one reference can be made to the variable.
Attempting to create two mutable references will fail:

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

results in a compile-time error:
```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

The first mutable borrow, `r1`, must last until it's used, but `r2` was created prior to `r1` seeing use.

This restriction allows for mutation in very controlled circumstances.
Most languages do not have this restriction, and many beginners to Rust struggle with the borrow checker as a result.
However, this prevents data races at compile time.

A data race is similar to a race condition, and happens when three behaviours occur:
 1. Two or more pointers access the same data at the same time
 2. At least one of the pointers is being used to write to the data
 3. There's no mechanism to sync the access to the data.

Data races cause undefined behaviour and can be difficlt to diagnose and fix at runtime.

Braces can be used to create new scopes, allowing for multiple mutable references:

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

There's a simiilar rule for mutable and immutable references.

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

causes a compile-time error.

```$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

A mutable reference cannot exist while there's an immutable reference to the same value.
Multiple immutable references are allowed as no modifications are made to the data, and the ability to read the data is not affected.

Note: the scope of a reference starts when it is created, and ends the last time it's used.
So this code will work:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

This works bbecause the last use of the immutable references occur before the mutable reference is created.
These scopes don't overlap, so the code is allowed.

Borrow errors can be frustrating, but the Rust compiler is pointing out potential bugs before they can happen.


## Dangling References

In other languages with pointers, it's easy to create *dangling pointers*: pointers that point to memory given to something else.
The Rust compiler guarantees that references will never be dangling.
If a reference exists to some data, the compiler ensures that data does not go out of scope before the reference to the data does.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

`dangle` creates a variable and a reference to that variable. 
At the end of `dangle`'s scope, `s` is dropped, and `&s` now references memory that may point to something else.
So `&s` would be a dangling reference, and rust produces a compile-time error:

```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

This error message referes to lifetimes, which will be discussed in detail in Chapter 10.
The key to the error is presented in this line:

```
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
```

which explains what happened as above.

## The rules of references

 - At any given time, a scope may have *either* one mutable reference *or* any number of immutable references.
 - References must always be valid

Next: another kind of reference - *slices*