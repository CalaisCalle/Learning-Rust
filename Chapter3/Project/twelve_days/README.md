# Twelve Days Notes

This is a bit of a bad attempt, imo.
First, I cheated: I made my first attempt at 2am, and was tocouldn't think of a good way to layo out the code, so I googled it.
While this isn't exactly the same as the code I found because I did this from memory, the layout is similar.

I should have realised early that you have two things going on: an ordinal number counting up ("first", "second", etc.), and then a list of gifts counting down.
The gifts count down from the value associated with the ordinal number.

This should have told me to use a loop within a loop, but I'd forgotten about arrays, so didn't think of a decent way of storing the values.
I probably would have hit it if I'd tried my hand at it first, but who knows.

The main issue I took with the code was in these lines:

```rust
if i > 0 && j == 0 {
    print!("and ")
}
```

This check is run on every iteration of the inner loop.
Easiest way to fix this is to just run the check in the outer loop, and always print the final line (it always gets printed anyway).

So now it looks like this:

```rust
    //<-for loop here->
    // Day one gift is printed on every iteration, with "and" added after the first.
    if i > 0 {
        print!("and ");
    }

    println!("{}\n", DAYS[0].1);
```

I also didn't know how to create an array of tuples as a constant (doing it as a variable is easy because of type inference).
Turns out you just need to "template" the tuple - I'd be interested to know if this is the compiler creating a new type.

End result:

```   Compiling twelve_days v0.1.0 (/home/max/Documents/Programming Projects/Learning-Rust/Chapter3/Project/twelve_days)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/twelve_days`
On the first day of Christmas, my true love gave to me...
A partridge in a pear tree!

On the second day of Christmas, my true love gave to me...
Two turtle Doves
and A partridge in a pear tree!

On the third day of Christmas, my true love gave to me...
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the fourth day of Christmas, my true love gave to me...
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the fifth day of Christmas, my true love gave to me...
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the sixth day of Christmas, my true love gave to me...
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the seventh day of Christmas, my true love gave to me...
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the eighth day of Christmas, my true love gave to me...
Eight maids a-milking
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the ninth day of Christmas, my true love gave to me...
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the tenth day of Christmas, my true love gave to me...
Ten lords-a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the eleventh day of Christmas, my true love gave to me...
Eleven pipers piping
Ten lords-a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!

On the twelth day of Christmas, my true love gave to me...
Twelve drummers drumming
Eleven pipers piping
Ten lords-a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese-a-laying
Five Golden Rings
Four calling birds
Three french hens
Two turtle Doves
and A partridge in a pear tree!
```

Final thing bothering me is the capitalisation, but oh well.
