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

## Second Attempt

I decided to use `print!` to print without ending a line:

```rust
fn main() {
    for num in 1..=100 {
        let mut printed: bool = false;

        if num % 3 == 0 {
            print!("Fizz");
            printed = true;
        }
        if num % 5 == 0 {
            print!("Buzz");
            printed = true;
        }
        
        if !printed {
            println!("{num}");
            continue;
        }

        println!();
    }
}
```

The weakness of this is that I have to find another way of printing the number if it's not a multiple of 3 or 5.
Again, this works, but I don't like the way the code looks.
I'd like to find a way of improving this further, but I'm not sure how.
Perhaps I should ask a colleague?

## ChatGPT's version
I ended up asking ChatGPT if it could write a Fizzbuzz program, to see if I could get an idea of improvements to make.
After asking it to improve the code from its initial output, this was its response:

```rust
fn main() {
    for i in 1..=100 {
        let result = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => i.to_string(),
        };
        println!("{}", result);
    }
}
```

This is superficially cool, and uses tuples in a way I hadn't considered, but would be annoying to update.
If a user wanted to extend the possible multiples (e.g. printing "bang" when a number is divisible by 7), then the programmer would have to account for (I think) $\sum_{k=0}^{n} {n \choose k}$ permutations.

N.B. I actually got that right, but also wrong. It's $2^n$ permutations, which I'm annoyed I didn't realise. 
What's cool is that $\sum_{k=0}^{n} {n \choose k} = 2^n$, which I might have known once, but forgotten.

So the match would get massive very quickly.