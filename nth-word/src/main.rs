fn main() {
    let n: u32 = 4;
    let phrase = String::from("foo bar baz ");
    let nth_word = get_nth_word(4, &phrase[..]);

    println!(
        "The {} word in \"{}\" is \"{}\"",
        get_ordinal(n),
        phrase,
        nth_word
    )
}

fn get_nth_word(n: u32, s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut word_start = 0;
    let mut space_count: u32 = 0;

    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            space_count += 1;

            if space_count == n {
                return &s[word_start..i];
            } else {
                word_start = i + 1;
            }
        }
    }

    ""
}

fn get_ordinal(n: u32) -> String {
    match (n % 10, n < 11 || n > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}
