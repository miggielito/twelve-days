fn main() {
    const NUM_DAYS: usize = 12;

    const PHRASES: [&str; NUM_DAYS] = [
        "A partridge in a pear tree",
        "Two turtledoves, and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    const ORDINALS: [&str; NUM_DAYS] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut end = 1;
    for day in ORDINALS {
        println!("On the {day} day of Christmas my True Love gave to me,");
        for i in (0..end).rev() {
            let phrase = PHRASES[i];
            println!("{phrase}");
        }
        end += 1;
        println!();
    }
}
