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
