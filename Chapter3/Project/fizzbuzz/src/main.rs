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
