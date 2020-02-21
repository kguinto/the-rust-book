use std::io::stdin;

fn main() {
    let temp_in_f = read_temp();

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

fn read_temp() -> f32 {
    let mut str_input;

    loop {
        println!("Enter a temperature in Fahrenheit: ");

        str_input = String::new();

        stdin()
            .read_line(&mut str_input)
            .expect("That's not a valid temperature.");

        match str_input.trim().parse() {
            Ok(num) => return num,

            Err(_) => {
                println!("\n{} is not a valid temperature. ", str_input.trim());
                continue;
            }
        };
    }
}
