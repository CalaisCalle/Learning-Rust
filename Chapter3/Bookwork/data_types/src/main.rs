fn main() {
    // parse can return different data types
    // variable assignment must be annotated.
    let guess = "42".parse().expect("Not a number!");
}
