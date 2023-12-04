use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// const RED: i32 = 12;
// const GREEN: i32 = 13;
// const BLUE: i32 = 14;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    // let mut game_count = 1;

    'skip_game: for line in reader.lines() {
        let mut l = line.unwrap();

        let mut idx = 0;
        for (i, c) in l.chars().enumerate() {
            if c == ':' {
                idx = i;
            }
        }

        l = l[idx + 1..].to_string();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let game_split = l.split(";");
        for toss in game_split {
            let dice = toss.split(",");
            for color in dice {
                let pieces = color.split_whitespace();
                let mut num = 0;
                for p in pieces {
                    if num == 0 {
                        print!("{}\n", p);
                        num = p.parse::<i32>().unwrap();
                    } else if num != 0 {
                        if p == "red" && num > red {
                            red = num;
                        } else if p == "green" && num > green {
                            green = num;
                        } else if p == "blue" && num > blue {
                            blue = num;
                        }
                        // if !find_color(num, p) {
                        //     game_count = game_count + 1;
                        //     continue 'skip_game;
                        // }
                    }
                }
            }
        }
        sum = sum + (red * green * blue);
        // game_count = game_count + 1;
    }
    print!("{}", sum);
    Ok(())
}

// fn find_color(num: i32, str: &str) -> bool {
//     (str == "red" && num <= RED)
//         || (str == "green" && num <= GREEN)
//         || (str == "blue" && num <= BLUE)
// }
