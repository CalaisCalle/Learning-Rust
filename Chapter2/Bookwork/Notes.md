# Notes on Chapter 2

Chapter introduces a few common Rust concepts:
 - `let`
 - `match`
 - methods
 - associated functions
 - external crates
 - More!

## Setting up a New Project

 Done in the same way as before, using Cargo:

 ```bash
 cargo new guessing_game
 cd guessing game
 ```

 Running `cargo build` or `cargo run` prints "Hello, World!" as before.

## Processing a guess

First step: getting user input.
Using the following code:

```rust
use std::io;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

`std::io` imports the io library into scope. `io` is part of the `std` (standard) library.

As before, `fn main() {` declares a new function with no parameters.
`println!()` prints things to the screen.

`let mut guess = String::new()` creates a new string to hold the user's guess.
`let` is used to create a variable:
```rust
let foo = bar;
```
this would create a variable called `foo`, and bind it to the `bar` variable.
Rust variables are immutable by default - they cannot be changed.
`mut` before a variable name makes the variable mutable.

The value `guess` is bound to is the result of the `String::new()` function, which returns a new string (imagine that!)
`::` indicates that `new` is an *associated function* of the `String` type: it is implemented on the type, rather than on a particular instance of `String`.
Many types have `new` function assocaited.

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

Without the `use std::io;` at the beginning, this could still be included using
```rust
std::io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

The `io::stdin()` function returns an instance of `std::io::Stdin` - a type representing the input of the terminal.
`.read_line(&mut guess)` calls `read_line` on the standard input handle to get input from the user.
The `&mut guess` argument is the string used to hold the user's input.
`&` indicates that it is a reference, which are also immutable by default.
`mut` changes it to mutable.

## Handling Potential Failure with the Result Type

`.expect()` is called on the result of `read_line()`.
When calling a method on the result of another, it's often a good idea to introduce a new line - rust ignores whitespace.

`read_line()` returns a `io::Result` instance.
Rust has several types called `Result`: generic `Result` and specific versions for submodules, like `io::Result`.
`Result` types are *enums* - types with a fixed set of values (*variants*).
`Result`s variants are `Ok` or `Err` - `Err` also contains info on why the operation failed.

`io::Result` instances have an `expect` method that can be called on them - this causes the program to crash and display the message displayed.
If `Result` is an `Ok` value, `expect` returns the value that `Ok` is holding - in this case, the number of bytes entered.
Without calling `expect`, the compiler will output a warning.

## Print values with `println!` Placeholders

`println!("You guessed: {}", guess);` has a placeholder, indicated by the `{}` - these will output the value on the right of the string.
You can use more than one placeholder:

```Rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```

## Generate a secret number:

Generating a secret number requires a crate, as the standard library does not include a random number generator.
One can be found [https://crates.io/crates/rand](here).
It can be added by using `cargo add rand`, or by anding `rand = "0.8.5` on a new line in the dependencies section of the toml file.
`0.8.5` is a Sematic Versioning number, shorthand for ^0.3.14 - a version with a public API compatible with version 0.3.14.
Cargo, when called with `cargo build/run`, will download rand and it's dependencies.

### Ensuring Reproducible Builds with the *Cargo.lock* file

When a project is built for the first time, Cargo figures out versions of dependencies that fit the criteria set in the *Cargo.lock* file, generated when the project is first build.
The project will remain at 0.8.5 unless explicitly upgraded in the Cargo.lock file.

### Updating a Crate to Get a New Version

`cargo update` will ignore the .lock file, and work out the latest version that fits the specs in the .toml.

## Generating a random number

Add `use rand::rng;` to the top of the file, then add

```rust
    ...
    println!("Guess the number!");

     let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    
    ...
```

`use rand::Rng` is a *trait* that defines methods implemented by random number generators.

`rand::thread_rng()` returns the particular RNG to be used - one local to current thread of execution.
`gen_range` is called on the RNG, and passed a range as an argument (two values separated by `..` defines a *range*). 
It will generate a number between the lower and upper bounds of the range.

## Comparing the Guess to the Secret Number

Add another statement at the top of the file:
`use std::cmp::Ordering`.

and 

```rust
    --snip--
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    --snip--
```

The first statement brings a type called `std::cmp::Ordering` into scope. 
`Ordering` is another enum, but with variants `Less` `Greater`, and `Equal`.

`cmp` compares two values and can be called on nanything that can be compared.
It takes a references to whatever the comparison is to be made to and returns a variant of the `Ordering` enum.

A `match` expression decides what to do next based on the variant.
The expression is made of *arms* - each has a pattern and the code that should be run if the expression fits that pattern.

Example: The `cmp` method between a guess of 50 and a secret value of 38 will return `Ordering::Greater`.
The `match` expression will execute the code on the `Ordering::Greater` arm.

However, the code above will not compile: the types are mismatched.
The secret number is a signed 32-bit int (`i32`), and the guess is a string.
Rust defaults to `i32`, unless the type info is otherwise specified.
To solve the issue, convert the values:

```Rust
    let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");
```

This creates a `shadow` of the previous value of `guess` with a new one.
`Shadowing` allows the reuse of variable names (covered in more detail in chap 3).
`guess` is bound to `guess.trim().parse()`, where `guess` in the expression is the original variable.
`trim()` eliminates any whitespace - pressing ENTER on user input adds a newline character to the string.
`parse()` parses a string into some kind of number - it can parse a variety of number types, so here the exact number type (`u32`) is specified.
The colon after `guess` tells Rust the exact type to use.

`parse()` might fail, so it returns a `Result` type, which can be treated with `expect()`.

Program now returns:
```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 47
Please input your guess.
47
You guessed: 47
You win!
```

## Allow multiple guesses with looping:

`loop` creates an infinite loop - everything within the `{}` will be run ad nauseum (haha latin sucks).
```rust
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
```

will run forever.
The loop can be exited using CTRL+C, or by entering a non-number (causing a crash).
To exit the loop, add a break statement:

```rust
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
```

(`{}` allows expressions and statements to be grouped together).
The program can also be made to ignore a non-number rather than crashing:
```rust
let guess: u32 = match guess.trim()
                        .parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        };
```
This switches the `expect` to a `match` expression, and is generally the way to handle an error.
A `match` expression is working on the `Result` type.
`Ok` will contain the `num` value, matching the `Ok(num)` pattern - this arm returns the num.
`Err` will contain error information, but it will be discarded.
The `_` is a catchall value - it will catch all `Err` values, regardless of what's in them, and discard it.
`continue` tells the loop to go the the next iteration.

Final step: remove `println!("The secret number is: {}", secret_number);`