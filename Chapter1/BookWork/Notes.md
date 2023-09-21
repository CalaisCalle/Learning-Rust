# Chapter 1 Notes

## Installing rustup

A C++ compiler and linker is needed for any operating system.
For Linux / macOS, there's probably one already.
For Windows, you need the C++ build tools for Visual Studio.
### Linux / macOS

In terminal, type:
`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

It may be necessary to reload the shell.
In bash this can be done with 
`source ~/.bashrc`

### Windows:

Go to `https://www.rust-lang.org/tools/install/`

Download the Visual Studio build tools:

`https://visualstudio.microsoft.com/downloads/?q=build+tools`

### Checking installation:

Use `rustc --version` in a terminal to check that Rust installed correctly.

## Hello, World!

There are two versions of this: one made by hand, and one made using cargo.
They are found in their own folders.