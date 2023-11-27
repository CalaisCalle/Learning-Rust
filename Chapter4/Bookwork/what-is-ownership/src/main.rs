fn main() {
    println!("Hello, world!");

    {
        let s = String::from("Hello");

        println!("{s}");
    }

    // Uncomment this line to cause an error: s is no longer in scope
    //println!("{s}");
}
