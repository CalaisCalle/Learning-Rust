fn main() {
    let mut s = String::from("hello world");

    let _hello = &s[0..5]; // slice from start to 5th index ("hello")
    let _world = &s[6..11]; // Slice from 6th to 11th positions ("world")

    let first = first_word(&s);
    println!("{first}");

    s.clear(); // s is empty, so first is no longer a valid slice
    // Uncommenting this causes an error
    // println!("{first}");

    /* String slices as parameters */
    // Making first_word take string slices as a parameter allows
    // someone to input a slice of the whole word:

    let s2 = String::from("why, hello there!");
    let second = first_word(&s2[5..]);
    println!("{second}");

    /* Using slices with other types */

    // A slice can be used with arrays of i32s
    let a : [i32; 5] = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// Using a string slice as a parameter makes the function more general.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
