# 3.3 Functions

One of the most important functions in the language has already been shown: the `main` function.
Functions are declared using the `fn` keyword.

In Rust, conventional function names use *snake_case* for function and variable names: All letters are lowercase and underscores separate words.

E.g.
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Functions are declared by entering the `fn` keyword followed by a function name and set of parentheses.
Curly braces tell the compiler where the function begins and ends.

Any defined function can be called using its name and a set of parentheses (`another_function();`).
`another_function` is declared after the main function, but can be used in main.
Unlike other languages, Rust does not care where functions are defined.

## Parameters

Functions can be defined to have parameters: special variables that are part of the function's signature.
IF a function has parameters, it can be provided with values for those parameters, called *arguments*.

For example, this version of `another_function` has  one parameter:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

When run, this program will produce
```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5
```

`another_function` takes a parameter called `x`, or type `i32`.
The value is then passed to `println!`.

In function signatures, the type of each parameter *must* be declared.
Requiring this in function definitions means the compiler almost never requires the annotation elsewhere.
The compiler can also give more useful errors.

Multiple parameters are separated in with commas:

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

This function has two parameters: `value` (type `i32`) and `unit_label` (type `char`).
It prints text containing both:

```
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

## Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.
Rust is an expression-based language, so the distinction between expressions and statements is important.
Other languages don't have the same distinctions. Here are Rust's.

 - **Statements** are instructions performing some action. They do not return a value.
 - **Expressions** evaluate to a resultant value.

Expressions can be a part of statements, but not the other way around.

Creating a value and assigning to a variable is a statement:

```rust
let y = 6;
```

Function definitions are also statement.
Statements do not return values, so a `let` statement cannot be assigned to another value:
```rust
fn main() {
    let x = (let y = 6);
}
```

will return an error:
```
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement
```

`let y = 6` does not return a value, so `x` cannot bind to anything.
In other languages (e.g. `C`), the assigment returns the value of the assignment.

Expressions evaluate to a value and make up the most of the rest of the code written in rust.
For example, `5 + 6` is an expression evaluating to and returning `11`.
Expressions can be a part of statements, too: the `6` in `let y = 6;` statement is an expression.

Calling a function or a macro, or creating a new scope block with curly braces, are all expressions.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

This expression

```rust
{
    let x = 3;
    x + 1
};
```

is a block that evaluates to 4.
y then binds to the `4`.
The `x + 1` line doesn't have a semicolon at the end - expressions don't needed them.
Adding a semicolon to the end of an expression turns in into a statement, which will not return a value.

## Functions with Return Values

Functions can return values to the code that calls them.

These values are not named, but their type must be declared after an arrow.
In Rust, the return value of the function is the same as the final expression in the body of the function.

A function can return early using the `return` keyword and specifying a value.
Most functions return the last expression implicitly.

Example:
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

The `five` function has no macros, functions, or `let` statements.
This is a perfectly valid function - it ends in an expression.
The function's return type is specified with the `-> i32` after the brackets.

The above code will output
```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

or close to it.

The `five` function has no parameters, and the body is just `5`.
The `5` is an expression whose value shall be returned;

A function with a return type and parameters will look like this:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

This program will print `The value of x is: 6`, but if a semi-colon is placed at the end of the expression, it results in an error:

```$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

The error `mismatched types` reveals the core issue.
The definition of `plus_one` says it will return an i32, but statements don't return a value.
This is represented by `()`, and nothing is returned.
This contradicts the function definition, producing the error.