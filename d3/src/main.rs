use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * loop through, find all numbers
 * store each number's digits (x, y)
 *
 * loop for each symbol
 * at the symbol, check if its surrounding (x, y) are
 * in the list of numbers are their coordinates
 */

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let f = BufReader::new(File::open(file_path).unwrap());

    let arr: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|char| char).collect())
        .collect();

    let mut sum = 0;
    for (i, row) in arr.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if arr[i][j].is_ascii_digit() {
                let mut should_add = false;

                let mut digits: Vec<u32> = vec![];
                let mut number = 0;

                while j < row.len() && arr[i][j].is_ascii_digit() {
                    digits.insert(0, arr[i][j].to_digit(10).unwrap());
                    //check to see if there is a symbol for this number

                    if !should_add {
                        should_add = check_for_symbol(arr.clone(), i, j, row.len() as isize);
                    }

                    j = j + 1;
                }

                //if this number should be added, do it
                if should_add {
                    if digits.len() == 1 {
                        sum = sum + digits[0];
                    } else {
                        let mut l = digits.len();
                        while l != 0 {
                            number = number + digits.pop().unwrap() * 10_u32.pow((l - 1) as u32);

                            l = l - 1;
                        }
                    }

                    sum = sum + number;
                }
            }
            j = j + 1;
        }
        print!("\nSUM -> {:?}\n", sum);

        println!("");
    }
}

fn check_for_symbol(arr: Vec<Vec<char>>, i: usize, j: usize, row_len: isize) -> bool {
    let x = j as isize;
    let y = i as isize;
    let mut found = false;
    if x - 1 >= 0 {
        found = match arr[i][j - 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    if x + 1 < row_len && !found {
        found = match arr[i][j + 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    if y - 1 >= 0 && !found {
        found = match arr[i - 1][j] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    if y + 1 < arr.len() as isize && !found {
        found = match arr[i + 1][j] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    // - -
    if x - 1 > 0 && y - 1 > 0 && !found {
        found = match arr[i - 1][j - 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    //++
    if x + 1 < row_len && y + 1 < arr.len() as isize && !found {
        found = match arr[i + 1][j + 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    //+-
    if x - 1 > 0 && y + 1 < arr.len() as isize && !found {
        found = match arr[i + 1][j - 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    //-+
    if x + 1 < row_len && y - 1 > 0 && !found {
        found = match arr[i - 1][j + 1] {
            '*' => true,
            '+' => true,
            '$' => true,
            '#' => true,
            '@' => true,
            '%' => true,
            '=' => true,
            '&' => true,
            '-' => true,
            '/' => true,
            _ => false,
        };
    }
    return found;
}
