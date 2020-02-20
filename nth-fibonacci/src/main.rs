use std::io::stdin;

fn main() {
    let mut n = 0;

    println!("Enter an integer:");

    read_int(&mut n);

    let nth_fib = get_nth_fibonacci(n);

    println!(
        "The {ordinal} fibonnaci number is {nth_fib}",
        ordinal = get_ordinal(n),
        nth_fib = nth_fib
    )
}

fn get_nth_fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2),
    }
}

fn get_ordinal(num: i32) -> String {
    match num {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", num),
    }
}

fn read_int(i: &mut i32) {
    let mut str_input = String::new();

    loop {
        stdin()
            .read_line(&mut str_input)
            .expect("That's not a valid integer.");

        match str_input.trim().parse() {
            Ok(num) => {
                *i = num;
                break;
            }

            Err(_) => continue,
        };
    }
}
