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
