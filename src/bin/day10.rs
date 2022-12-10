use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use itertools::Itertools;

#[derive(Debug)]
enum Op {
    Add(i32),
    Noop,
}

impl From<String> for Op {
    fn from(line: String) -> Self {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "addx" => Self::Add(words[1].parse::<i32>().expect("could not parse operand")),
            "noop" => Self::Noop,
            &_ => panic!("unknown operand..."),
        }
    }
}

struct Cpu {
    reg: i32,
}

#[derive(Debug)]
struct Trace {
    trace: Vec<i32>,
}

impl Cpu {
    fn new(reg: i32) -> Self {
        Self { reg }
    }

    fn execute(&mut self, ops: Vec<Op>) -> Trace {
        let mut trace = Trace::new();

        for op in ops {
            match op {
                Op::Add(val) => {
                    trace.push(self.reg);
                    trace.push(self.reg);
                    // increment value for the 3rd cycle...
                    self.reg += val;
                }
                Op::Noop => trace.push(self.reg),
            }
        }

        trace
    }
}

impl Trace {
    fn new() -> Self {
        Self { trace: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.trace.push(val)
    }

    fn get_sig_strength(&self, pos: usize) -> i32 {
        (pos as i32) * self.trace[pos - 1]
    }
}

fn task1(lines: Vec<String>) {
    let ops = lines.into_iter().map(|line| Op::from(line)).collect_vec();

    let mut cpu = Cpu::new(1);

    let trace = cpu.execute(ops);

    let strengths = (20..=220)
        .step_by(40)
        .map(|pos| trace.get_sig_strength(pos))
        .sum::<i32>();

    println!("1. sum of the first 6 sig strengths: {}", strengths);
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct Screen {
    pixels: [[char; WIDTH]; HEIGHT],
}

impl Screen {
    fn generate_image(trace: Trace) -> Self {
        let mut pixels: [[char; WIDTH]; HEIGHT] = [['.'; WIDTH]; HEIGHT];

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                match (col as i32) - trace.trace[row * WIDTH + col] {
                    -1 | 0 | 1 => pixels[row][col] = '#',
                    _ => (),
                }
            }
        }

        Self { pixels }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGHT {
            writeln!(f, "{}", self.pixels[i].iter().collect::<String>())?;
        }
        Ok(())
    }
}

fn task2(lines: Vec<String>) {
    let ops = lines.into_iter().map(|line| Op::from(line)).collect_vec();

    let mut cpu = Cpu::new(1);

    let trace = cpu.execute(ops);

    println!("{:?}", trace);

    let crt = Screen::generate_image(trace);

    println!("{}", crt);
}

fn main() {
    let file_name = PathBuf::from("./input/day10");

    match read_lines(&file_name) {
        Ok(lines) => {
            let lines = lines.flatten().collect_vec();
            task1(lines.clone());
            task2(lines);
        }
        Err(e) => println!("Error during reading: {:?}", e),
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
