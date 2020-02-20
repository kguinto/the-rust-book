use std::io::stdin;

fn main() {
    println!("Enter a temperature in Fahrenheit:");

    let mut temp_in_f: f32 = 0.0;

    read_float(&mut temp_in_f);

    let temp_in_c = f_to_c(temp_in_f);

    println!(
        "{temp_in_f}{deg}F is {temp_in_c}{deg}C",
        temp_in_f = temp_in_f,
        deg = 176 as char,
        temp_in_c = temp_in_c
    )
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn read_float(f: &mut f32) {
    let mut str_input = String::new();

    loop {
        stdin()
            .read_line(&mut str_input)
            .expect("That's not a valid temperature.");

        match str_input.trim().parse() {
            Ok(num) => {
                *f = num;
                break;
            }

            Err(_) => continue,
        };
    }
}
