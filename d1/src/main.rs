use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let mut f = 0;
        let mut b = 0;
        for i in 0..l.len() {
            if f == 0 {
                let substring = l.as_str()[..i].to_string();
                let contains_word = sub_has_word(substring);
                if contains_word > 0 {
                    f = contains_word;
                }
            }
            if l.as_bytes()[i].is_ascii_digit() && f == 0 {
                let c = l.as_bytes()[i] as char;
                f = c.to_digit(10).unwrap();
            }

            if b == 0 {
                let substring = l.as_str()[(l.len() - 1 - i)..l.len()].to_string();
                let contains_word = sub_has_word(substring);
                if contains_word > 0 {
                    b = contains_word;
                }
            }
            if l.as_bytes()[l.len() - 1 - i].is_ascii_digit() && b == 0 {
                let c = l.as_bytes()[l.len() - 1 - i] as char;
                b = c.to_digit(10).unwrap();
            }
            if f != 0 && b != 0 {
                break;
            };
        }

        // DO THE SUM
        if f != 0 && b != 0 {
            sum += (f * 10) + b;
        } else if b != 0 {
            sum += (b * 10) + b;
        } else {
            sum += (f * 10) + f;
        }
    }
    println!("{}\n", sum);

    Ok(())
}

fn sub_has_word(str: String) -> u32 {
    if str.contains("one") {
        return 1;
    } else if str.contains("two") {
        return 2;
    } else if str.contains("three") {
        return 3;
    } else if str.contains("four") {
        return 4;
    } else if str.contains("five") {
        return 5;
    } else if str.contains("six") {
        return 6;
    } else if str.contains("seven") {
        return 7;
    } else if str.contains("eight") {
        return 8;
    } else if str.contains("nine") {
        return 9;
    }
    return 0;
}
