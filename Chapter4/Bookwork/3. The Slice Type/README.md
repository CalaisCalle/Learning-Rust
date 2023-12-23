# The Slice Type

*Slices* are references to a contiguous sequence of elements in a collection, rather than the whole collection.
A slice is a kind of reference: it does not have ownership.

Consider a problem: take a string of space-separated words and return the first word in the string.
If no space is found, then the whole string is one word and must be returned.

Without using slices, the function could work with references.
One method could be to return the index of the end of the word, indicated by a space:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

if no spaces are found, the lengthh of the string is returned.
Because the function goes through the string element-by-element, the string must be converted into an array of bytes using the `as_bytes` method.

```rust
    let bytes = s.as_bytes()
```

and an iterator to iterate over the array of bytes using the `iter` method

```rust
    for (i, &item) in bytes.iter().enumerate() {...}
```

Iterators are discussed in more detail in chapter 13.
For now, consider that `iter` is a method that returns each element in a collection and `enumerate` wraps the result and returns each element as part of a tuple instead.
The first element of the tuple if the index, and the second element is a reference to the element.

A pattern can be used to destructure the tuple (discussed further in Chapter 6).
In the `for` loop, the pattern contains `i` for the index of the byte and `&item` for the reference to the byte.
The loop then checks if the byte represents a space using the byte-literal syntax (`b' '`).
If a space is found, then its position is returned.
Otherwise, the length of the string is returned using `s.len()`.

There's a problem with this method: the `usize` value returned is only meaningful in the context of an `&String`.
If the referenced string is cleared or deleted, then it will no longer be valid in future.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

The program compiles without errors, even if `word` is used after `s.clear()`, as `word` is a separate value from the string.

Managing synchronisation between the `word` and `s` variables becomes tedious - even more so if another function was created to find the second string.
Such a function would require indices for both the beginning and end of a string, resulting in 3 unrelated values floating about.

Rust's solution to this is string slices.

## String Slices

A *string slice* is a reference to part of a `String`:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Rather than reference the entire string, `hello` references a portion of the string, specified by the `[0..5]` syntax.
Slices are created using a range within brackets: `[starting_index..ending_index]`.
Note: the range is not inclusive, so the `ending_index` must be one greater than the last position in the slice.
Internally, the slice structure stores the start position and the end of the slice (`ending_index` - 1).
So `world` is a slice that contains a pointer to a byte at index 6, with a length of `5`.

When starting from 0 using the `..` range syntax, the 0 can be dropped (`slice = &s[..2]`).
Equally, if the slice ends at the last byte of the string, then the trailing number can be dropped (slice = &s[2..]).
Finally, both values can be dropped to take a slice of the whole string (`slice = &s[..]`).

 > Note: String slice range indices must occur at valid UTF-8 character boundaries.
 > A UTF-8 character can be multiple bytes.
 > Creating a string slice in between one of the bytes in a multi-byte chracter will cause an error.
 > More thorough discussion is found in Chapter 8 - Storing UTF-8 Encoded Text with Strings.

So, rewriting `first_word` to return a string slice (`&str`):

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' [
            return &s[0..i];
        ]
    }

    &s[..]
}
```

The index of the space is found in the same way as the first function.
However, this time a string slice is returned when a space is found, using the index of the space as the end position of the slice.
Now, `first_word` returns a single value tied to the underlying data.
The value consists of a reference to the starting point of the slice, and the number of elements the slice contains.

This approach would also work well for returning the second word in the function.

The compiler will make sure that references to the `String` remain valid.
Previously, the code was logically incorrect but wouldn't produce errors.
Instead, the problems would occur when the program was run.
Slices make this problem impossible as the compiler would find the issue much sooner:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

creates a compiler error:
```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

From the borrowing rules: if there's an immutable reference to something, then a mutable reference cannot be created.
Because `clear` truncates the string, it needs a mutable reference.
The `println` after `clear` requires the immutable reference in `word` to still be active at this point.
The compiler knows this, so when `clear` is called, the immutable reference has not yet been discarded.
So when `clear` is called, the immutable reference is still active, and the borrow checked disallows the creation of a mutable reference for `clear`.
The end result of this is a failure to compile

## String Literals as Slices

String literals are stored inside of the binary.

When using the following code

```rust
    let s = "hello world";
```

the type of `s` is a `&str`: a string slice.
Specifically, it's a string slice that points to a specific part of the binary.
This happens to be why string literals are immutable.

--- 

## String Slices as Parameters

Another improvement can be made to `first_word`: rather than taken a reference to a string, it can take a string slice:

```rust
fn first_word(s: &str) -> &str {...}
```

This version of the function can be used on both `String` and `&str` string slices.
IT takes advantage of *deref coercions*: a feature covered further in the Chapter 15 section "Implicit Deref Coercions".

Defining a function to take a string slice instead of a reference to a string makes the function much more general without losing functionality.

---

## Other Slices

String slices are specific to strings, but there's also a more general slice type.
Take this array:

```rust
    let a = [1, 2, 3, 4, 5];
```

A slice can be taken of array `a` to refer to only a part of it:

```rust
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
```

The slice has type `&[i32]`, and works the same way as a string slice:
it stores a reference to the first element, and a length.
This kind of slice is used with many sorts of collections, discussed further in Chapter 8.