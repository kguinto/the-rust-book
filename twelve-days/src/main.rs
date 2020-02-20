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

fn get_ordinal(num: i32) -> String {
    match num % 10 {
        1 => {
            if num == 11 {
                "11th".to_string()
            } else {
                format!("{}st", num)
            }
        }
        2 => {
            if num == 12 {
                "12th".to_string()
            } else {
                format!("{}nd", num)
            }
        }
        3 => {
            if num == 13 {
                "13th".to_string()
            } else {
                format!("{}rd", num)
            }
        }
        _ => format!("{}th", num),
    }
}
