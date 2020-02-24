use std::string::String;

//! Converts strings to pig latin.
//! The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
//! Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

enum ReadState {
    ReadingStartingCluster,
    ReadingRestOfWord,
}

pub fn pig_latinize(s: String) -> String {
    let mut result = String::from("");

    let mut starting_cluster = String::from("");
    let mut read_state = ReadState::ReadingStartingCluster;

    for ch in s.chars() {
        match ch == ' ' {
            true => match read_state {
                ReadState::ReadingRestOfWord => {
                    result.push_str(&starting_cluster);
                    result.push_str("ay ");
                    starting_cluster = String::from("");
                    read_state = ReadState::ReadingStartingCluster;
                }
                _ => {}
            },

            false => match read_state {
                ReadState::ReadingStartingCluster => {
                    let ch_is_consonant = !is_vowel(ch) || (ch == 'y' && starting_cluster == "");
                    match ch_is_consonant {
                        true => starting_cluster.push(ch),
                        false => {
                            if starting_cluster == "" {
                                starting_cluster.push('h');
                            }
                            result.push(ch);
                            read_state = ReadState::ReadingRestOfWord;
                        }
                    }
                }
                ReadState::ReadingRestOfWord => result.push(ch),
            },
        }
    }
    match read_state {
        ReadState::ReadingRestOfWord => {
            result.push_str(&starting_cluster);
            result.push_str("ay");
        }
        _ => {}
    }

    result
}

fn is_vowel(ch: char) -> bool {
    match ch {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'y' => true,
        _ => false,
    }
}
