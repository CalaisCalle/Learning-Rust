# Notes on Fizzbuzz

A classic interview question, I thought this would be a good way to check fundamentals.
My initial attempt looked something like this:

```rust
fn main() {
    let mut out = String::new();
    
    for num in 1..100 {
        out.clear();
        if num % 3 == 0 {
            out.push_str("Fizz");
        }
        if num % 5 == 0 {
            out.push_str("Buzz");
        }
        
        if out.len() == 0 {
            println!("{num}");
            continue;
        }

        println!("{out}");
    }
}
```

Which works, although it wasn't perfect first time.
I'm not keen on the idea of taking an initial string and pushing "Fizz" and "Buzz" to it.
It's only FizzBuzz, so doesn't need to much optimisation, but I thought that just printing "Fizz" or "Buzz", instead of concatenating the string, would reduce memory and processing.