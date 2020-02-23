use ordnl;
use std::io::stdin;

fn main() {
    let n = read_n();

    let nth_fib = get_nth_fibonacci(n);

    println!(
        "The {ordinal} fibonnaci number is {nth_fib}",
        ordinal = ordnl::of_u32(n),
        nth_fib = nth_fib
    )
}

fn get_nth_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2),
    }
}
fn read_n() -> u32 {
    let mut str_input;

    loop {
        println!("Enter an whole number: ");

        str_input = String::new();

        stdin()
            .read_line(&mut str_input)
            .expect("That's not a valid whole number");

        match str_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("\n{} is not a valid whole number ", str_input.trim());
                continue;
            }
        };
    }
}
