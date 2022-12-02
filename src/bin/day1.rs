use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::collections::BinaryHeap;

fn main() {
    let filename = PathBuf::from("./input/day1");

    match read_lines(&filename) {
        Ok(lines) => {
            let mut calories = 0_u32;
            let mut max_calories = 0_u32;
            for line in lines.flatten() {
                if line.is_empty() {
                    if calories > max_calories {
                        max_calories = calories;
                    }
                    calories = 0;
                } else {
                    match line.parse::<u32>() {
                        Ok(n) => {
                            calories += n;
                        }
                        Err(e) => {
                            panic!("unexpected input: {}", e);
                        }
                    }
                }
            }

            println!(
                "1. maximum calories carried by a single elve: {}",
                max_calories
            );
        }
        Err(e) => {
            panic!("could not open file: {}", e);
        }
    }

    match read_lines(&filename) {
        Ok(lines) => {
            let mut cal = 0_u32;
            let calories : u32 = lines.into_iter()
                .map(move |line| {
                    let line = line.expect("Expected line to be valid");
                    if line.is_empty() {
                        None
                    } else {
                        match line.parse::<u32>() {
                            Ok(n) => Some(n),
                            Err(e) => panic!("Error parsing the input: {}", e),
                        }
                    }
                })
                .filter_map(move |number| {
                    match number {
                        Some(n) => {
                            cal += n;
                            None
                        },
                        None => {
                            let out = cal;
                            cal = 0;
                            Some(out)
                        },
                    }
                })
                .collect::<BinaryHeap<u32>>() // insert into sorted collection
                .into_iter()
                .take(3)
                .sum();

            println!("2. combined calories of 3 elves with most caleries: {}", calories);
        }
        Err(e) => {
            panic!("could not open file: {}", e);
        }
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}

