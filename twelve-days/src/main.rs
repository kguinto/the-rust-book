use ordnl;

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
            ordnl::of_u32(i + 1)
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
