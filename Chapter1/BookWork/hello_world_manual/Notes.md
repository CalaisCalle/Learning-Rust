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
$ ./hello_world
Hello, world!
```

*Windows*
```
> rustc main.rs
> .\main.exe
Hello, world!
```