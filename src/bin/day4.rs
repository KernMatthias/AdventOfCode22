use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn main() {
    let file_name = PathBuf::from("./input/day4");

    // first part
    match read_lines(&file_name) {
        Ok(lines) => {
            let count = lines
                .into_iter()
                .flatten()
                .flat_map(move |line| {
                    let regex = Regex::new(r"(\d+)-(\d+)").unwrap();
                    let numbers = regex
                        .captures_iter(&line)
                        .flat_map(move |entry| {
                            let begin = entry[1].parse::<u32>().expect("");
                            let end = entry[2].parse::<u32>().expect("");

                            Some((begin, end))
                        })
                        .collect::<Vec<_>>();

                    assert!(numbers.len() == 2);

                    if numbers[0].0 >= numbers[1].0 && numbers[0].1 <= numbers[1].1
                        || numbers[1].0 >= numbers[0].0 && numbers[1].1 <= numbers[0].1
                    {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            println!("1. number of fully contained pairs: {count}");
        }
        Err(e) => {
            panic!("Could not open file: {e}");
        }
    }

    // second part
    match read_lines(&file_name) {
        Ok(lines) => {
            let count = lines
                .into_iter()
                .flatten()
                .flat_map(move |line| {
                    let regex = Regex::new(r"(\d+)-(\d+)").unwrap();
                    let numbers = regex
                        .captures_iter(&line)
                        .flat_map(move |entry| {
                            let begin = entry[1].parse::<u32>().expect("");
                            let end = entry[2].parse::<u32>().expect("");

                            Some((begin, end))
                        })
                        .collect::<Vec<_>>();

                    if numbers[0].0 >= numbers[1].0 && numbers[0].0 <= numbers[1].1
                        || numbers[0].1 >= numbers[1].0 && numbers[0].1 <= numbers[1].1
                        || numbers[1].0 >= numbers[0].0 && numbers[1].0 <= numbers[0].1
                        || numbers[1].1 >= numbers[0].0 && numbers[1].1 <= numbers[0].1
                    {
                        Some(())
                    } else {
                        None
                    }
                })
                .count();

            println!("2. number of partially contained pairs: {count}");
        }
        Err(e) => {
            panic!("Could not open file: {e}");
        }
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
