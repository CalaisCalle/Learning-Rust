# What is Ownership

*Ownership* is a set of rules governing Rust's memory management.
Common approaches to memory management include Garbage Collection, and user-managed memory.
Rust uses a third option: A system of ownership with a set of rules that the compiler can check.
If any rule is violated, the program will not compile.
It is a *zero-cost* abstraction: None of the features of ownership slow down the program as it runs.

Ownership is new to many programmers, but as experience is gained, the easier it becomes to naturally develop safe, efficient code.

## The Stack and the Heap

```
The stack and the heap are different parts of memory available to the program at runtime.
The stack is a last-in, first-out structure: values are stored in the order the stack receives them, and removed in the reverse order.
An analogy would be a stack of plates.
Adding data to the stack is called *pushing onto the stack*, while removing data is called *popping off the stack*.
All data stored on the stack must have a known, fixed size.

Data with an unknown size at compile time must be stored on the *heap*. 
When data is added to the heap, the memory allocator requests a certain amount of space, and finds a large enough portion of empty memory.
This portion is marked as being in use, and a *pointer* (memory address of that location) is returned.
The process is called *Allocating on the heap* (or *allocating*, for short).
The pointer itself is a known, fixed size and so can be stored on the stack, but to get the actual data, the program must follow the pointer.

Pushing to the stack is faster than allocating on the heap: a push to the stack never has to search for a place for the data: it's always at the top of the stack.
In contrast, allocating to the heap requires that the allocator finds a space big enough for the data, and perform bookkeeping to prepare for the next allocation.

Accessing data on the heap is also slower, as a pointer must be followed to get there.
Modern CPUs are faster if they jump around less in memory i.e. the data it's working on are close to other data.

When a program calls a function, the values passed to the function (which can include heap pointers) and the function's local variables are pushed to the stack.
Once the function has finished processing, those values get popped of the stack.

Keeping track of what parts are using what data on the heap, minimizing duplicate data on the heap, and cleaning up unused data on the heap are all problems addressed by ownership.
```

## Ownership Rules:

 1. Each value is Rust has an *owner*
 2. There can only be one owner at a time
 3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

The *scope* of some variables can be looked at for an example of ownership.
A scope is simply the range within which an item in the program is valid.
E.g. 
```rust
let s = "hello";
```

`s` refers to a string literal: the value of the string is hardcoded into the text of the program.
`s` is valid from the point at which it is declared, until the end of the current scope.
```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

## The `String` Type

A more complex type than those covered in *Data Types* is required to illustrate ownership rules.
Those types were of a known size and so easy to store on and pop off the stack when the scope is over.
It's also trivial to copy basic types to make a new, independent instance if the same value needs to be used in a different scope.
`String` is an example of a type whose data is stored on the heap and makes a good example.

String literals (which have a fixed size and can be stored on the stack) aren't suitable for every situation.
Not every string value can be known, and the literals themselves are immutable.
For example, the program cannot know user input at compile time.
These situations are why the `String` type exists.
The `String` type manages heap-allocated data, and so can store text of an unknown size.
A `String` can be created from a string literal using the `from` function:

```rust
    let s = String::from("hello");
```

`::` operators access the namespace of the left operand (`String` in this instance) and access the function or property (right operand).
This is discussed more in Chapter 5, under "Method Syntax".

A `String` can be mutated:

```rust
    let mut s = String::from("hello");
    
    s.push_str(", world!");
    
    println!("{}", s);
    
```

This will output `hello, world!`.

Unlike a string literal, a `String` can be mutated.
This is down to the differences in how each type deals with memory.

## Memory and Allocation

The size of a string literal is known at compile time, so the text can be hardcoded into the final executable.
This makes them fast and efficient, but this stems from the literal's immutability.
The compiler can't put a blob of memory into the binary for each piece of text of unknown, mutable size.

The `String` type must allocate an amount of meory on the heap, unknown at compile time.
As such:
 - The memory must be requested from the allocator at runtime
 - There must be a way of returning this memory to the allocator when the program finishes with the `String`.

`String::from` requests the memory required when called - this is pretty universal in programming languages.
In some languages with *Garbage Collectors (GCs)*, the GC keeps track of and cleans up unused memory.
In languages without a GC, it is the programmers responsibility to identify when memory is no longer in use and call code to explicitly free it.
This has historically been a challenging problem to solve.
Done too early, and the program will be left with an invalid variable.
If it's allocated twice, that's also a bug.
One `allocate` must be paired with exactly one `free`.

Rust chooses instead to return the memory automatically once the owning variable goes out of scope.

```Rust
    {
        let s = String::from("hello");

        // Do stuff with S
    }
```

The natural point to return the memory to the allocator occurs when `s` goes out of scope.
When a variable goes out of scope, Rust calls a special function called `drop`, where the author of `String` can put the code to return the memory.
Rust calls `drop` automatically at the closing curly bracket (end of the scope).

 - N.B: In C++, this pattern of deallocating resources at the end of an objects lifetime is called Resource Acquisition Is Initialisation (RAII).

This pattern is simple, but has large ramifications.
Unexpected behaviour can occur in more complicated situations where multiple variables need to use heap-allocated data.

## Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in rust.
For example, with integers:

```rust
    let x = 5;
    let y = x;
```

This binds a value of 5 to the variable x, then makes a copy of the value in x and assigns that value to y.
There are now two variables equal to 5, which is possible because an integer is a simple type of a fixed size.
The two values of 5 are both pushed onto the stack.

Now, the string version:

```rust
    let s1 = String::from("hello");
    let s2 = s1;
```

This looks similar, but behaves differently.
`let s2 = s1` does not make a copy of the value in `s1.
Understanding what is happening requires an understanding of what's happening at a lower level.

A `String` is made up of three parts: a pointer to memory on the heap that holds the contents of the string; a length; and a capacity. This data is stored on the stack.
The length is the amount of memory, in bytes, occupied by the contents of the `String`.
The capacity is the total memory, in bytes, that has been received by the allocator.
The difference between length and capacity matters, but not right now.

When `S1` is copied into `S2`, the `String` data is copied: the pointer, len, and capacity.
The data on the heap is not copied, so `s1` and `s2` both point to the same memory.
If Rust did copy the memory on the heap as well as the memory on the stack, the cost of the operation would be much greater.

When a variable goes out of scope, Rust calls the `drop` function and cleans up the memory pointed to by that variable.
But both `s1` and `s2` point to the same memory, so when they go out of scope, the both try to free the same memory.
This is called a `double free` error and is a major memory safety bug that can lead to memory corruption, and lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1;`, rust considers `s1` as no longer valid.
Thus, `s1` cannot be used after this assignment:

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
```

causes an error:
```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

The concept of copying only the pointer, length, and capacity, without copying the data, probably sounds like a shallow copy.
But Rust also invalidates the first variable, making this a `move`.
In this example, `s1` is said to have been `moved` into `s2`.

Since only `s2` is valid when it goes out of scope, it alone will free the memory.
There is also a design choice implied here: **Rust will never automatically create a deep copy** of the program data.
Therefore, any *automatic* copying can be assumed to be inexpensive.

## Variables and Interacting with Clone

