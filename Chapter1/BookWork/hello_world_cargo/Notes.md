# Hello_world_cargo notes

Cargo is Rust's build system and package manager.
It handles many of the tasks for the programmer, including compiling code, downloading library dependencies, and building those libraries.
 - Libraries needed by the code are dependencies

The simplest programs have no dependencies.
The hello_world program built manually would only have used the compiling part of Cargo.

The vast majority of projects use Cargo - the book assumes this to be true of its reader.
Cargo comes installed with Rust - the install can be checked with 

```bash
cargo --version
```
Anything other than a version number means its not installed correctly.

## Creating a project with cargo

In the terminal/cmd line, navigate to the projects directory and type
```bash
cargo new hello_world_cargo
cd hello_world_cargo
```

The first line creates a new direcotry called `hello_world_cargo`, as well as files within that directory.
Within the directory, cargo generates two files, and one directory:
```bash
Cargo.toml  src
src/main.rs
```
It will also initialise a new Git repository, along with a new .gitignore file.

The TOML (Tom's Obvious, Minimal Language) file format ios Cargo's configuration format.
It looks something like this
```
[package]
name = "hello_world_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Everything under "\[package\]" is used to configurre a package.
The next 3 lines set the config info Cargo needs to compile the program.
Everything under "\[Dependencies\]" is a project dependency.

Packages of Rust code are referred to as *Crates* - none will be needed here.

The `src/main.rs` file will contain the hello world program:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo expects source files to live inside the src directory; the top level of the project is only for READMEs, license info, config files, and anything not related to code.

## Building and Running a Cargo project

Navigate to the hello_world_cargo directory and type
```bash
cargo build
```

Output (on Ubuntu Linux 22.04)
```bash
   Compiling hello_world_cargo v0.1.0 ([...]Chapter1/BookWork/hello_world_cargo/hello_world_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
```

This will build in `target/debug/hello_world_cargo` (`target\debug\hello_world_cargo.exe` on windows)

Executable run as before, or can be compiled and run in:
```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_world_cargo`
Hello, world!
```

Cargo also provides a command called `cargo check` - this checks the quality of the code, without compiling.
`check` is much faster than `build`, so this can be useful for large projects.

## Building for Release

Use `cargo build --release` to compile a release version of the program.
This creates an executable in `target/release`, and turns on optimizations - this may increase build time.