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

        let split = l.split("|");
        let mut l_cards: Vec<i32> = vec![];
        let mut r_cards: Vec<i32> = vec![];
        for (idx, str) in split.enumerate() {
            if idx == 0 {
                let left_side = str.split(":");
                for (i, s) in left_side.enumerate() {
                    if i == 1 {
                        let my_cards = s.split(" ");
                        for st in my_cards {
                            let n: i32 = st.parse().unwrap_or(-200);
                            l_cards.push(n);
                        }
                    }
                }
            }
            if idx == 1 {
                let my_cards = str.split(" ");
                for st in my_cards {
                    let n: i32 = st.parse().unwrap_or(-500);
                    r_cards.push(n);
                }
            }
        }
        let mut num_matches: i32 = -1;
        for c in r_cards {
            if l_cards.contains(&c) {
                num_matches = num_matches + 1;
            }
        }
        if num_matches > -1 {
            let p = num_matches as u32;
            sum = sum + (2_i32.pow(p));
        }
    }
    println!("{}", sum);
    Ok(())
}
