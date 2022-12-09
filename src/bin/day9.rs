use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Rope<const LEN: usize> {
    parts: [Pos; LEN],
}

impl<const LEN: usize> Rope<LEN> {
    fn new() -> Self {
        Self {
            parts: [Pos::new(0, 0); LEN],
        }
    }

    fn update_tail(&mut self) {
        for idx in 1..LEN {
            let head = self.parts[idx - 1];
            let tail = &mut self.parts[idx];

            match (head.x - tail.x, head.y - tail.y) {
                (0, 0) => (),
                (-1 | 1, 0) | (0, -1 | 1) | (1, 1) | (-1, -1) | (-1, 1) | (1, -1) => (),
                (2, 0) => tail.x += 1,
                (-2, 0) => tail.x -= 1,
                (0, 2) => tail.y += 1,
                (0, -2) => tail.y -= 1,
                (2, 1) | (1, 2) | (2, 2) => {
                    tail.x += 1;
                    tail.y += 1;
                }
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    tail.x -= 1;
                    tail.y -= 1;
                }
                (2, -1) | (1, -2) | (2, -2) => {
                    tail.x += 1;
                    tail.y -= 1;
                }
                (-2, 1) | (-1, 2) | (-2, 2) => {
                    tail.x -= 1;
                    tail.y += 1;
                }
                (_, _) => panic!(
                    "this state should not be reachable: part[{idx}] = {:?}, part[{}] = {:?}",
                    head,
                    idx - 1,
                    tail
                ),
            }
        }
    }

    fn apply(&mut self, m: Move, visited: &mut HashSet<Pos>) {
        for _ in 0..m.cnt {
            match m.dir {
                Dir::Up => self.parts[0].y += 1,
                Dir::Down => self.parts[0].y -= 1,
                Dir::Left => self.parts[0].x += 1,
                Dir::Right => self.parts[0].x -= 1,
            }
            self.update_tail();
            visited.insert(self.parts[LEN - 1].clone());
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s.chars().next().expect("no char?") {
            'D' => Self::Down,
            'U' => Self::Up,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid direction..."),
        }
    }
}

struct Move {
    dir: Dir,
    cnt: u32,
}

impl Move {
    fn new(dir: Dir, cnt: u32) -> Self {
        Self { dir, cnt }
    }
}

impl From<String> for Move {
    fn from(s: String) -> Self {
        let content = s.split_whitespace().collect_vec();

        let dir = Dir::from(content[0]);
        let cnt = content[1]
            .parse::<u32>()
            .expect("could not parse to int...");

        Move::new(dir, cnt)
    }
}

fn task1(lines: Vec<String>) {
    let mut set: HashSet<Pos> = HashSet::new();

    let mut rope: Rope<2> = Rope::new();

    lines
        .into_iter()
        .map(|line| Move::from(line))
        .for_each(|m| rope.apply(m, &mut set));

    println!("1. number of visited fields: {}", set.len());
}

fn task2(lines: Vec<String>) {
    let mut set: HashSet<Pos> = HashSet::new();

    let mut rope: Rope<10> = Rope::new();

    lines
        .into_iter()
        .map(|line| Move::from(line))
        .for_each(|m| rope.apply(m, &mut set));

    println!("2. number of visited fields, rope length 10: {}", set.len());
}

fn main() {
    let file_name = PathBuf::from("./input/day9");

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
