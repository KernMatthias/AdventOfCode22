use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Crate {
    id: char,
}

impl Crate {
    fn new(id: char) -> Self {
        Self { id }
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.id)
    }
}

impl TryFrom<&str> for Crate {
    type Error = ();

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        if val.starts_with("[") && val.ends_with("]") && val.len() == 3 {
            return Ok(Crate::new(val.chars().nth(1).expect("")));
        }

        Err(())
    }
}

#[derive(Debug)]
struct Row {
    pub crates: Vec<Option<Crate>>,
}

impl Row {
    fn new(crates: Vec<Option<Crate>>) -> Self {
        Self { crates }
    }
}

impl From<String> for Row {
    fn from(row: String) -> Self {
        let row = row
            .chars()
            .chunks(4)
            .into_iter()
            .map(move |chunk| {
                let mut chars = chunk.collect::<String>();
                chars.truncate(3);
                chars
            })
            .map(move |chars| Crate::try_from(chars.as_str()))
            .map(move |freight| match freight {
                Ok(freight) => Some(freight),
                Err(_) => None,
            })
            .collect_vec();

        Self::new(row)
    }
}

#[derive(Debug)]
struct Storage {
    pub stacks: Vec<Vec<Crate>>,
}

impl Storage {
    fn new(stacks: Vec<Vec<Crate>>) -> Self {
        Self { stacks }
    }

    fn apply(&mut self, m: Move) {
        for _ in 0..m.cnt {
            let item = self.stacks[m.from_idx as usize - 1]
                .pop()
                .expect("invalid move!");
            self.stacks[m.to_idx as usize - 1].push(item);
        }
    }

    fn apply_v2(&mut self, m: Move) {
        let len = self.stacks[m.from_idx as usize - 1].len();
        let items = self.stacks[m.from_idx as usize - 1].split_off(len - m.cnt as usize);
        self.stacks[m.to_idx as usize - 1].extend_from_slice(&items);
    }

    fn get_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().expect("invalid state").id.clone())
            .collect::<String>()
    }
}

impl From<Vec<Row>> for Storage {
    fn from(rows: Vec<Row>) -> Self {
        let mut stacks: Vec<Vec<Crate>> = vec![Vec::new(); rows[0].crates.len()];

        rows.into_iter().for_each(|row| {
            row.crates.into_iter().enumerate().for_each(|(i, freight)| {
                if let Some(freight) = freight {
                    stacks[i].push(freight);
                }
            });
        });

        Storage::new(stacks)
    }
}

impl From<Vec<String>> for Storage {
    fn from(input: Vec<String>) -> Self {
        let rows = input
            .into_iter()
            .skip(1)
            .map(move |row| Row::from(row))
            .collect_vec();

        Storage::from(rows)
    }
}

#[derive(Debug)]
struct Move {
    cnt: u8,
    from_idx: u8,
    to_idx: u8,
}

impl Move {
    fn new(cnt: u8, from_idx: u8, to_idx: u8) -> Self {
        Self {
            cnt,
            from_idx,
            to_idx,
        }
    }
}

impl From<String> for Move {
    fn from(line: String) -> Self {
        let mut words = line.split_whitespace();

        assert_eq!(words.next().expect("invalid input"), "move");
        let cnt = words
            .next()
            .expect("invalid input")
            .parse::<u8>()
            .expect("invalid input");

        assert_eq!(words.next().expect("invalid input"), "from");
        let from_idx = words
            .next()
            .expect("invalid_input")
            .parse::<u8>()
            .expect("invalid input");

        assert_eq!(words.next().expect("invalid input"), "to");
        let to_idx = words
            .next()
            .expect("invalid_input")
            .parse::<u8>()
            .expect("invalid input");

        Move::new(cnt, from_idx, to_idx)
    }
}

fn main() {
    let file_name = PathBuf::from("./input/day5");

    // task 1
    match read_lines(&file_name) {
        Ok(mut lines) => {
            let mut input: Vec<String> = lines
                .by_ref()
                .take_while(|ref line| match line {
                    Ok(line) => !line.is_empty(),
                    Err(e) => panic!("{e}"),
                })
                .map(|line| line.as_ref().expect("").clone())
                .collect();

            input.reverse();

            let mut state = Storage::from(input);

            lines
                .into_iter()
                .map(|line| Move::from(line.expect("invalid input")))
                .for_each(|m| state.apply(m));

            println!("1. Top row: {}", state.get_top());
        }
        Err(e) => {
            panic!("Error reading files {e}");
        }
    }

    // task 2
    match read_lines(&file_name) {
        Ok(mut lines) => {
            let mut input: Vec<String> = lines
                .by_ref()
                .take_while(|ref line| match line {
                    Ok(line) => !line.is_empty(),
                    Err(e) => panic!("{e}"),
                })
                .map(|line| line.as_ref().expect("").clone())
                .collect();

            input.reverse();

            let mut state = Storage::from(input);

            lines
                .into_iter()
                .map(|line| Move::from(line.expect("invalid input")))
                .for_each(|m| state.apply_v2(m));

            println!("2. Top row with CrateMover 9001: {}", state.get_top());
        }
        Err(e) => {
            panic!("Error reading files {e}");
        }
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
