fn main() {
    const FIRST_PART:&str = "On the";
    const SECOND_PART:&str = "day of Christmas, my true love sent to me";
    
    const GIFTS: [&str;12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree"
    ];

    const DAYS: [&str;12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    for (idx, day) in DAYS.iter().enumerate() {
        println!("{} {} {}", FIRST_PART, day, SECOND_PART);
        let mut count = idx;
        loop {
            println!("{}", GIFTS[11-count]);
            if count == 0 {
                break;
            }
            count -= 1;
        }
        println!()
    }
    println!("{}", GIFTS[11]);


}
