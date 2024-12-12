use std::collections::HashMap;
use std::fs::{self};
use std::io;

fn read_file_quick(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn main() {
    let file_path = "src/adventofcode.com_2024_day_1_input.txt";
    let file = read_file_quick(file_path).unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut i = 0;
    for line in file.lines() {
        for word in line.split_whitespace() {
            if i % 2 == 0 {
                left_list.push(word.parse().expect("Word should be a number"));
            } else {
                right_list.push(word.parse().expect("Word should be a number"));
            }
            i += 1;
        }
    }

    left_list.sort();
    right_list.sort();

    let mut distance: i32 = 0;
    let mut similarity: i32 = 0;
    let mut left_count: HashMap<i32, i32> = HashMap::new();
    let mut right_count: HashMap<i32, i32> = HashMap::new();
    for (i, j) in left_list.iter().zip(right_list.iter()) {
        left_count.entry(*i).and_modify(|counter| *counter += 1).or_insert(1);
        right_count.entry(*j).and_modify(|counter| *counter += 1).or_insert(1);
        let difference: i32 = i - j;
        distance += difference.abs();
    }
    for (k, v) in left_count.iter() {
        let right_option = right_count.get(k);
        let right_value: &i32;
        match right_option {
            Some(value) => right_value = value,
            None => right_value = &0
        };
        similarity += v * k * right_value;
    }
    println!("The distance is {distance}");
    println!("The similarity is {similarity}");
}
