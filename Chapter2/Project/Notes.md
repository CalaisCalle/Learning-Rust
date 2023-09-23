# Notes on quadratic solver

Decided to try and make a simple quadratic equation solver.
Starting out: I do not know how to find an exponent or square root, so I found out and imported the library.
This turned out to be `std::f64`, so no crate required

`f64` has a `sqrt` method, so creating a value and taking its sqrt, then printing it:

```rust
use std::f64;

fn main() {
    let x: f64 = 1.5;

    println!("{}", x.sqrt())
}
```

I imagine similar libraries exist for f32, so I switched to that (no need for 64-bit precision).

Next, ask user for a, b, and c:
```rust
    let mut entry = String::new();

    println!("Enter a:");

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

    let mut a: f64 = entry.trim().parse().expect("ARGH!");

    println!("Enter b:");

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line");

        
    let mut b: f64 = entry.trim().parse().expect("ARGH!");
```
But this caused an error:
```bash
thread 'main' panicked at 'ARGH!: ParseFloatError { kind: Invalid }', src/main.rs:23:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

I got it working by adding new strings for b and c - I think parse() actually transforms the thing it's parsing into a new type. 
Why?
I will consider this a tomorrow problem. 

## Calculate b**2 - 4ac

Turns out Rust has two power functions: powi and powf for raising to an integer and float, respectively.
I'll use powi here.

It also turns out that rust will complain if you multiply a float by an int:

```rust
33 |     let determinant = b.powi(2) - 4 * a * c;
   |                                     ^ no implementation for `{integer} * f64`
```
I kind of get why, but like, why?
Looked it up on StackOverflow:
Integrals don't hae a fractional part, and floating point types can only represent a limited number of integers exactly.
Rust would have to decide which piece of data was less important (the int or the float), or raise an error.
For a systems programming language, it's better to raise an error - systems behaviour should be deterministic.

Fixed it by using `4_f64` instead.

I don't want to both with complex numbers for now, so want to check for +ve determinant.
I guessed what `if` statements look like, and how to exit early, and got this:
```rust
    if (determinant < 0_f64) {
        println!("No real solutions!");
        return;
    }
```
Turns out that's pretty close, but I don't need the () .

It also turns out that `cargo fix --bin <project name>` doesn't like uncommitted changes, but it does fix all warnings if it has fixes.

So, calculating results:

```rust
    let result1 = (-b + determinant) / (2_f64 * a);
    let result2 = (-b - determinant) / (2_f64 * a);

    println!("Results: {}, {}", result1, result2);
```

Which i DEFINITELY wrote first try. DEFINITELY didn't forget to specify the type of the literals. Nuh-uh.

There's a lot that could be improved here:
 - Calculate using complex numbers
 - Find a way to reuse input strings
 - Make the code cleaner with less copy-pasting

I think it'll be worth coming back to this one once I've progressed further.