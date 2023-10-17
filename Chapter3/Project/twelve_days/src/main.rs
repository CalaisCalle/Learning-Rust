fn main() {
    const DAYS: [(&str, &str); 12] = [
        ("first", "A partridge in a pear tree!"),
        ("second", "Two turtle Doves"),
        ("third", "Three french hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five Golden Rings"),
        ("sixth", "Six geese-a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eighth", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords-a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelth", "Twelve drummers drumming"),
    ];

    // Count up through the ordinal numbers
    for i in 0..12 {
        let ordinal = DAYS[i].0;
        println!("On the {ordinal} day of Christmas, my true love gave to me...");

        // Count down through the gifts (except day one)
        for j in (1..=i).rev() {
            println!("{}", DAYS[j].1);
        }

        // Day one gift is printed on every iteration, with "and" added after the first.
        if i > 0 {
            print!("and ");
        }

        println!("{}\n", DAYS[0].1);
    }
}
