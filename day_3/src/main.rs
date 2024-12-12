use regex::Regex;
use std::fs::{self};
use std::io;

fn read_file_quick(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn main() {
    let file_path: &str = "data/adventofcode.com_2024_day_3_input.txt";
    let file = read_file_quick(file_path).unwrap();

    let re_part1 = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut sum_mul: i32 = 0;
    for mat in re_part1.find_iter(&file) {
        let re_number = Regex::new(r"[0-9]{1,3}").unwrap();
        let numbers: Vec<_> = re_number
            .find_iter(&file[mat.start()..mat.end()])
            .map(|m| m.as_str())
            .collect();
        let x: i32 = numbers[0].parse().unwrap();
        let y: i32 = numbers[1].parse().unwrap();
        sum_mul += x * y;
    }
    println!("{:?}", sum_mul);
}
