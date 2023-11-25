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