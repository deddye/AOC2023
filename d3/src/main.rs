use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

//loop through
//at each symbol check each location around it
//while spaces right and left are not '.'
//countinue getting numbers to make your number
//maybe store them in an array then turn to decimal

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let f = BufReader::new(File::open(file_path).unwrap());

    let arr: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|char| char).collect())
        .collect();

    for (i, row) in arr.iter().enumerate() {
        for (i, col) in row.iter().enumerate() {
            print!("{} ", col);
        }
        println!("");
    }
}
