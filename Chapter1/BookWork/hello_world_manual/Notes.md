# Hello_world_manual notes

created using:

```bash
$ mkdir hello_world_manual
$ cd hello_world_manual
```

In Windows, this would be:

```powershell
> mkdir hello_world_manual
> cd hello_world_manual
```

Or just do it in file explorer or whatever

## Writing/Running a Rust Program

Make a source file with the extension `.rs` - in this case, hello_world.rs.

Enter:
```rust
fn main() {
    println!("Hello, world!");
}
```

Save the file and then, in the terminal window, type

*Linux*
```bash
$ rustc hello_world.rs
$ ./hello_worldprintln!("Hello, world!");
```

*Windows*
```
> rustc main.rs
> .\main.exe
Hello, world!
```

### Anatomy of a Rust Program

```rust
fn main() {

}
```

These lines define a function called `main`.
The `main` function is always the first code that runs in every Rust program.
The first line (`fn main() {`) declares a function that returns nothing and takes nothing as a parameter.
The function body is wrapped in curly braces `{}`, which are required around all function bodies.

An automatic rust formatting tool can be used to keep consistent style across projects: `rustfmt`.
Rust style is to indent with 4 spaces, not a single tab.

```rust
println!("Hello, world!");
```
T

This line does the actual work of the program.
`println!` calls a macro, not a function.
A function call would not have a `!`.
The difference is covered later.

The line is ended in a semicolon: this indicates the ned of an expression.

### Compiling and running are separate steps.

Like C and C++, the program must be compiled before it can be run.
This is done here by calling `rustc` in the terminal and passing in the source file:
```bash
rustc hello_world_manual.rs
```
This will create an executable in the same folder as the source file.
