use crate::common::day::Day;
use crate::days::day0::Day0;
use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod common;
mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world! Starting the advent");
    let args = std::env::args().collect::<Vec<String>>();
    let parsed_input: Result<(String, String), Box<dyn std::error::Error>> = match args.as_slice() {
        [_, input_folder_arg, day_arg] => {
            let input_folder = Path::new(input_folder_arg);
            if !input_folder.is_dir() {
                return Err("Input folder does not exist".into());
            }

            let input_file = input_folder.join(format!("{}.input", day_arg));
            if !input_file.is_file() {
                return Err("Input file does not exist for the specified day".into());
            }

            let mut buffer = String::new();
            File::open(&input_file)?.read_to_string(&mut buffer)?;
            Ok((day_arg.to_string(), buffer))
        }
        _ => Err("Usage: <program> <input_folder> <day>".into()),
    };

    let (day, input) = parsed_input?;

    let day: Box<dyn Day> = match day.as_str() {
        "0" => Box::new(Day0 { input }),
        "1" => Box::new(Day1 { input }),
        "2" => Box::new(Day2 { input }),
        "3" => Box::new(Day3 { input }),
        "4" => Box::new(Day4 { input }),
        "5" => Box::new(Day5 { input }),
        "6" => Box::new(Day6 { input }),
        _ => {
            return Err(format!("No implementation known for day: {}", day.as_str()).into());
        }
    };
    println!("Result part 1: {}", day.part1()?);
    println!("Result part 2: {}", day.part2()?);
    Ok(())
}
