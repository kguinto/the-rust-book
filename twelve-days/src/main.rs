fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me",
            get_ordinal(i + 1)
        );

        for j in (0..(i + 1)).rev() {
            print!("\t{gift}", gift = gifts[j as usize]);

            match j {
                1 => print!(", and\n"),
                0 => print!(".\n\n"),
                _ => print!(", \n"),
            }
        }
    }
}

fn get_ordinal(n: i32) -> String {
    match (n % 10, n < 11 || n > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}
