# Notes on Chapter 3 - Common Programming Concepts

Chapter covers basic concepts found in almost every programming language:
 - Basic types
 - Functions
 - Comments
 - Control Flow

*Keywords*: rust has a set of keywords reserved for language-use only.
These cannot be used for variable or function names, and eachh has a special meaning.
A list appears in Appendix A of the book.

## Variables and Mutability

By default, variables are immutable - a nudge to write code to take advantage of Rust's safety and easy concurrency.
If a variable is immutable, then a value bound to a name cannot be changed.
For example:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

will not compile, and produces and error:

```bash
   Compiling variables v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Bookwork/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to previous error
```

The compiler will help find errors to help improve the safety of the program.
This error says that an attempt was made to assign a second value to an immutable variable.

If one part of the code operates on the assumption that a value never changes, and it then does change, it's possible to create a bug.
Thus the compiler will ensure that an immutable variable really never changes.
It is a tool to make reasoning throught the code easier.

Mutability can be useful, and make the code more convenient to write.
Variables can be made mutable by declaring `mut` in from of the variable name.

Changing the previous code:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

allows the program to compile, and x is now allowed to changed.

### Constants

Constants are values bound to a name that are not allowed to change.
Unlike variables, constants are **Always** immutable.
They are declared using `const`, and must be annotated with a type (The data types section covers type annotations).

Constants can be declared in any scope, making them useful for values used in many parts of the code.
Constants may only be set to a constant expression - not something that can be computed at runtime.

E.g. 

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

By convention, constant names are all uppercase, with underscores between words.
The compiler can evaluate a limited set of operation at compile time: the value can be written in a way that's easier to understand than e.g. 10800.

Constants are valid for the program's entire lifecycle in their declaring scope.
This makes them useful for values required by multiple parts of the program.
Naming hardcoded values will help future programmers understand the meaning of the  value.

### Shadowing

A new variable can be declared with the same name as a previous variable - called shadowing.
Shadowing means that the compiler will see the second variable when using the shadowed name.
The second variable overshadows the first, taking any uses of the name to mean itself.
It does this until either it itself is shadowed, or the scope ends.

Shadowed variables are declared using `let` with an existing variable name:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

This will bind x to a vlaue of 5, then to the original value + 1.
Within the inner scope, the third let creates a shadow and multiplies the previous value by 2.
Once the program exits that scope, x returns to being 6.

Shadowing differs from `mut`: it effectively creates a new variable when using `let` again.
The type of the variable can be changed while using the same name.
E.g.

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```

The first of these will a string type, and the second a number type.
However, using `mut` will give a an error:

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```


