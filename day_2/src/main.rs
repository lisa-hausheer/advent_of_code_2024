use std::error::Error;
use std::fs::{self, File};
use std::io;

fn read_file_quick(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/adventofcode.com_2024_day_2_input.txt";
    // let file_path = "data/example_data.txt";
    let file = read_file_quick(file_path).unwrap();

    let mut safe_report_count = 0;

    for line in file.lines() {
        let mut previous_option: Option<i32> = None;
        let mut is_increasing_option: Option<bool> = None;
        let mut is_safe = true;
        for word in line.split_whitespace() {
            let current_number: i32 = word.parse()?;
            match previous_option {
                None => previous_option = Some(current_number),
                Some(previous_number) => {
                    // Check the absolute difference
                    let difference = previous_number - current_number;
                    let difference_abs = difference.abs();

                    if difference_abs > 3 || difference_abs < 1 {
                        is_safe = false;
                    }

                    // Check the order stays the same
                    match is_increasing_option {
                        None => {
                            if difference > 0 {
                                is_increasing_option = Some(false);
                            } else if difference < 0 {
                                is_increasing_option = Some(true);
                            }
                        }
                        Some(is_increasing) => {
                            if (difference > 0 && is_increasing)
                                || (difference < 0 && !is_increasing)
                            {
                                is_safe = false;
                            }
                        }
                    };

                    // Update previous_number
                    previous_option = Some(current_number);
                }
            };
        }
        if is_safe == true {
            safe_report_count += 1;
        }
    }

    println!("The number of safe reports is {safe_report_count}.");
    Ok(())
}
