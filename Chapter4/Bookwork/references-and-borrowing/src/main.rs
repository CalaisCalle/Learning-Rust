fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // s1 can be used here as it was previously passed by reference
    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {
    s.len() 
} // calculate_length does not have ownership of the variable s
// When s goes out of scope, it is not dropped as a result.

// Trying to change the value of a borrowed variable results in an error:
// Uncommenting this function will cause a compile-time error.
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }