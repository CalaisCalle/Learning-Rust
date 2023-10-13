# 3.5 Control Flow

Running code and running code repeatedly based on a condition are the basics of most programming languages.
In Rust, the flow of execution is most commonly controlled using `if` expressions and loops.

## `if` Expressions

`if` expressions allows code branching on a condition.
If the condition is met, then a block of code is run.

E.g. 

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

`if` expressions start with the keyword `if`, followed by a condition - e.g. whether a number is less than 5.
A block of code is placed immediately after the condition - this block executes if the conditions is true.
Blocks of code within the `if` expressions are sometimes called *arms* (a la `match` expressions).

Optionally, an `else` condition can be included - this gives an alternate block of code to run if the condition evaluates to false.
If no `else` block is provided, and the condition is -ve, then the program will just skip the `if` block.

When run as above, the output looks like this:
```bash
   Compiling control_flow v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Bookwork/control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/control_flow`
condition was true
```