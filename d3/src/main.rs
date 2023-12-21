use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
    pub fn to_string(&self) -> String {
        format!("({}, {}) ", &self.x, &self.y)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, c: &Coordinate) -> bool {
        if self.x == c.x && self.y == c.y {
            return true;
        }
        false
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}
struct Number {
    number: u32,
    coordinates: Vec<Coordinate>,
}

impl Number {
    pub fn new(number: u32, coordinates: Vec<Coordinate>) -> Self {
        Number {
            number,
            coordinates,
        }
    }
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s = s + &format!("NUMBER -> {}\n", &self.number);
        s = s + &format!("Coordinates -> ");
        for coord in &self.coordinates {
            s = s + &format!("{}", coord.to_string());
        }
        s = s + &format!("\n\n");
        s
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let f = BufReader::new(File::open(file_path).unwrap());

    let arr: Vec<Vec<char>> = f
        .lines()
        .map(|l| l.unwrap().chars().map(|char| char).collect())
        .collect();
    let mut numbers: Vec<Number> = vec![];
    let mut sum = 0;
    for (i, row) in arr.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if arr[i][j].is_ascii_digit() {
                let mut num_struct = Number::new(0, vec![]);
                let mut digits: Vec<u32> = vec![];
                let mut number = 0;

                while j < row.len() && arr[i][j].is_ascii_digit() {
                    digits.insert(0, arr[i][j].to_digit(10).unwrap());
                    let c = Coordinate::new(j, i);
                    num_struct.coordinates.push(c);
                    j = j + 1;
                }
                let mut l = digits.len();

                while l != 0 {
                    number = number + digits.pop().unwrap() * 10_u32.pow((l - 1) as u32);

                    l = l - 1;
                }
                num_struct.number = number;
                numbers.push(num_struct);
            }
            j = j + 1;
        }
    }

    for (i, row) in arr.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if arr[i][j] == '*' {
                let ratio;

                let gear = Coordinate::new(j, i);
                let gear_perim = check_for_gear(&gear, arr.len(), row.len());
                let mut found_num: Vec<u32> = vec![];
                for g in gear_perim.iter() {
                    for n in numbers.iter() {
                        if n.coordinates.contains(g) {
                            if !found_num.contains(&n.number) {
                                found_num.push(n.number);
                            }
                        }
                    }
                }
                if found_num.len() == 2 {
                    ratio = found_num[0] * found_num[1];
                    sum = sum + ratio;
                }
            }
            j = j + 1;
        }
    }

    print!("\n\nSUM -> {:?}\n", sum);
}

fn check_for_gear(c: &Coordinate, arr_len: usize, row_len: usize) -> Vec<Coordinate> {
    let lt = (c.x - 1) as isize >= 0 && (c.y - 1) as isize >= 0;
    let l = (c.x - 1) as isize >= 0;
    let lb = (c.x - 1) as isize >= 0 && (c.y + 1) < arr_len;

    let t = (c.y - 1) as isize >= 0;
    let b = c.y + 1 < arr_len;

    let rt = c.x + 1 < row_len && (c.y - 1) as isize >= 0;
    let r = c.x + 1 < row_len;
    let rb = c.x + 1 < row_len && c.y + 1 < arr_len;

    let mut gear_coords: Vec<Coordinate> = vec![];
    if l {
        let cd = Coordinate::new(c.x - 1, c.y);
        gear_coords.push(cd)
    }
    if lt {
        let cd = Coordinate::new(c.x - 1, c.y - 1);
        gear_coords.push(cd)
    }
    if lb {
        let cd = Coordinate::new(c.x - 1, c.y + 1);
        gear_coords.push(cd)
    }
    if t {
        let cd = Coordinate::new(c.x, c.y - 1);
        gear_coords.push(cd)
    }
    if b {
        let cd = Coordinate::new(c.x, c.y + 1);
        gear_coords.push(cd)
    }
    if rt {
        let cd = Coordinate::new(c.x + 1, c.y - 1);
        gear_coords.push(cd)
    }
    if r {
        let cd = Coordinate::new(c.x + 1, c.y);
        gear_coords.push(cd)
    }
    if rb {
        let cd = Coordinate::new(c.x + 1, c.y + 1);
        gear_coords.push(cd)
    }

    gear_coords
}

// fn main() {
// let args: Vec<String> = env::args().collect();
//
// let file_path = &args[1];
// let f = BufReader::new(File::open(file_path).unwrap());
//
// let arr: Vec<Vec<char>> = f
// .lines()
// .map(|l| l.unwrap().chars().map(|char| char).collect())
// .collect();
//
// let mut sum = 0;
// for (i, row) in arr.iter().enumerate() {
// let mut j = 0;
// while j < row.len() {
// if arr[i][j].is_ascii_digit() {
// let mut should_add = false;
//
// let mut digits: Vec<u32> = vec![];
// let mut number = 0;
//
// while j < row.len() && arr[i][j].is_ascii_digit() {
// digits.insert(0, arr[i][j].to_digit(10).unwrap());
//
// if !should_add {
// should_add = check_for_symbol(arr.clone(), i, j, row.len() as isize);
// }
//
// j = j + 1;
// }
//
// if should_add {
// let mut l = digits.len();
// while l != 0 {
// number = number + digits.pop().unwrap() * 10_u32.pow((l - 1) as u32);
//
// l = l - 1;
// }
//
// sum = sum + number;
// }
// }
// j = j + 1;
// }
// print!("\nSUM -> {:?}\n", sum);
// }
// }
//
// fn check_for_symbol(arr: Vec<Vec<char>>, i: usize, j: usize, row_len: isize) -> bool {
// let x = j as isize;
// let y = i as isize;
// let mut found = false;
// if x - 1 >= 0 {
// found = match arr[i][j - 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// if x + 1 < row_len && !found {
// found = match arr[i][j + 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// if y - 1 >= 0 && !found {
// found = match arr[i - 1][j] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// if y + 1 < arr.len() as isize && !found {
// found = match arr[i + 1][j] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// //- -
// if x - 1 > 0 && y - 1 > 0 && !found {
// found = match arr[i - 1][j - 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// //++
// if x + 1 < row_len && y + 1 < arr.len() as isize && !found {
// found = match arr[i + 1][j + 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// //+-
// if x - 1 > 0 && y + 1 < arr.len() as isize && !found {
// found = match arr[i + 1][j - 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// -//+
// if x + 1 < row_len && y - 1 > 0 && !found {
// found = match arr[i - 1][j + 1] {
// '*' => true,
// '+' => true,
// '$' => true,
// '#' => true,
// '@' => true,
// '%' => true,
// '=' => true,
// '&' => true,
// '-' => true,
// '/' => true,
// _ => false,
// };
// }
// return found;
// }
//
