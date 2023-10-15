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

If the number is set to 7, then the output changes:
```
   Compiling control_flow v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Bookwork/control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/control_flow`
condition was false
```

It's important to note that the condition must be a Boolean, or else an error is produced.
This is in contrast to languages like Ruby, Javascript, and Python: Rust does not try to convert non-Boolean types to Boolean.

This code:
```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```
will not compile.

To check if a number is not 0, then the if expression must be changed.
```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

This code prints "number was something other than zero".

### Handling multiple conditions with `else if`

Multiple conditions can be used by combining `if` and `else` with `else if`:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

which should give something close to the following output:
```
   Compiling control_flow v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Bookwork/control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/control_flow`
number is divisible by 3
```

On execution, the program checks each expression and executes the first block of code where the condition is true.
Only the block for the first `true` condition is executed - the rest are not.
As a result, even though 6 is also divisible by 2, "number is divisible by 2" is not printed.

Using too many `else-if` expressions can clutter code, so more than one might be an indicator that the code requires refactoring.
Chapter 6 explains the `match` expression in more detail.

### Using `if` in a `let` statement.

`if` is an expression, so can be used on the right side of a `let` expression to assign the outcome of a variable.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

The `number` variable is bound to a value based on the outcome of the `if` expression.
In this case, it will be bound to the value `5`.
(This is similar to the ternary operator in other languages)

The value of the whole `if` relies on which block of code is executed.
Blocks of code return the value of the last expression: `5` and `6` are both expressions, so are returned based on the outcome of the condition.
This means that the expressions in both arms must evaluate to the same type (`i32` in the above example.)

So, this won't compile:
```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```
As the types in the true and false arms (`string` and `i32`, respectively) are mismatched.
Requiring the types to match allows the compiler to verify that `number` is the same type everywhere in the code.


## Repetition with Loops

Rust provides several *loops*, which execute a body of code within the loop more than once.
Rust has three types of loop: `loop`, `while`, and `for`.

### Repeating Code with `loop`

`loop` will tell Rust to execute a bit of code repeatedly forever, or until it's told to stop.
For example, 
```rust
    loop {
        println!("again!");
    }
```
will execute forever, and the program will need to be interrupted with ctrl+c.

To break the loop, place a `break` keyword somewhere within the loop to tell the program to stop looping.
This was done in the guessing game from Chapter 2, to end the game.

Another keyword, `continue`, can be used to skip over any remaining code in a loop iteration and start the next iteration.

### Returning Values from Loops

One use of a loop is to retry and operation that might fail (e.g. checking if a thread has completed it's job).
The result of an operation may need to be passed out of the loop.
To do this, the value to return can be placed just after the `break` expression:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

This code initialises a variable named `counter` to `0`.
A variable named `result` is declared to hold the value returned from a loop.
Each loop iteration adds `1` to the loop, and checks if the value is equal to `10`.
If it is, loop `break`s with the value `counter*2`.
Post loop, a semicolon declares the end of the `result` assignment expression.

### Loop Labels to Disambiguate Between Multiple Loops

When a program has loops within loops, `break` and `continue` apply to the innermost loop at that point.
Optionally, a loop can be given a label.
`break` and `continue` can then be used with this label to break that loop.
Loop labels must begin with a single quote:

```rust
let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
```

The outer loop has the label `counting_up`, and counts from 0 to 2.
The inner loop (no label) counts down from 0 to 9.
The first `break` (no label) will apply to the inner loop.
`break 'counting_up` exits the outer loop.

The output:
```
Compiling control_flow v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Bookwork/control_flow)
Finished dev [unoptimized + debuginfo] target(s) in 0.18s
    Running `target/debug/control_flow`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

## Conditional Loops with `while`

Loops may need to evalue a condition - while the condition is `true`, the loop runs.
Once the condition becomes `false`, the program should call `break`.
This can be done with a `loop` and some `if` expressions, but it's a common enough pattern to have it's own type of loop: `while`.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

The above program starts with a number 3, and loops until that number hits 0.
Using `while` eliminates a lot of nesting compared to using `loop`, `if`, `else`, and `break`.
While a condition evalues to `true`, the loop runs.

## Looping Through a Collection with `for`

A `while` loop can be used to loop over the elements of a collection:
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

This code counts up through elements in an array, starting at index 0.
There is a check to make sure the loop exits if the index is greater than the number of elements.

However, the code is error-prone: a panic could occur if the index value or test condition is incorrect.
E.g. if the array `a` was updated to contain 4 elements, but the condition was not updated.
It is also slow, as the compiler adds code to check that the index is within the array's bounds.

A `for` loop is more concise, and safe:
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
It produces the same output as the `while` loop, but eliminates the possibility of going beyond the array bounds.
If the collection was changed, the loop could remain the same.

`for` loops are the most commonly used loop construct in Rust.
They're also used to run code a certain number of times (a la the countdown code from earlier).
This can be done using a `Range`, provided by the standard library, that generates a series of numbers between a range of numbers.
The countdown would look like this:
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
```
