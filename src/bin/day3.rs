use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn main() {
    let file_name = PathBuf::from("./input/day3");

    // part one
    match read_lines(&file_name) {
        Ok(lines) => {
            let cumulated_prio: u32 = lines
                .into_iter()
                .flatten()
                .map(move |line| {
                    let first_compartment = &line[..line.len() / 2];
                    let second_compartment = &line[line.len() / 2..];

                    let mut prio = 0_u32;
                    for c in first_compartment.chars().unique() {
                        if second_compartment.chars().unique().contains(&c) {
                            if c.is_lowercase() {
                                prio += u32::from(c as u8 - b'a') + 1_u32;
                            } else {
                                prio += u32::from(c as u8 - b'A') + 27_u32;
                            }
                        }
                    }
                    prio
                })
                .sum();

            println!("1. Cumulated Priority of duplicate items is: {cumulated_prio}");
        }
        Err(e) => {
            panic!("Error reading from file: {e}");
        }
    }

    // part two
    match read_lines(&file_name) {
        Ok(lines) => {
            let cumulated_prio: u32 = lines
                .into_iter()
                .flatten()
                .map(move |line| {
                    let a: BTreeSet<char> = BTreeSet::from_iter(line.chars());
                    a
                })
                .chunks(3)
                .into_iter()
                .map(move |mut group| {
                    let a = group.next().expect("no a");
                    let b = group.next().expect("no b");
                    let c = group.next().expect("no c");

                    // get the intersection of the sets
                    let key = (&(&(a) & &(b)) & &(c))
                        .into_iter()
                        .next()
                        .expect("no key?");

                    if key.is_lowercase() {
                        u32::from(key as u8 - b'a') + 1_u32
                    } else {
                        u32::from(key as u8 - b'A') + 27_u32
                    }
                })
                .sum();

            println!("2. Cumulated Priority of badge items is: {cumulated_prio}");
        }
        Err(e) => {
            panic!("Error reading from file: {e}");
        }
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
