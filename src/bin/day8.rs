use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use itertools::Itertools;

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Self { map }
    }

    // there has to be a better solution but I'm too sleepy rn
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let mut visible = true;
        for dx in 0..x {
            if self.map[y][dx] >= self.map[y][x] {
                visible = false;
            }
        }

        if visible {
            return visible;
        }

        visible = true;

        for dx in x + 1..self.map[y].len() {
            if self.map[y][dx] >= self.map[y][x] {
                visible = false;
            }
        }

        if visible {
            return visible;
        }

        visible = true;
        for dy in 0..y {
            if self.map[dy][x] >= self.map[y][x] {
                visible = false;
            }
        }

        if visible {
            return visible;
        }

        visible = true;

        for dy in y + 1..self.map.len() {
            if self.map[dy][x] >= self.map[y][x] {
                visible = false;
            }
        }

        visible
    }

    fn get_num_visible_trees(&self) -> usize {
        let mut cnt: usize = 2 * self.map[0].len() + 2 * (self.map.len() - 2);

        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[y].len() - 1 {
                if self.is_visible(x, y) {
                    cnt += 1;
                }
            }
        }

        cnt
    }

    fn get_vis_score(&self, x: usize, y: usize) -> usize {
        if x == 0 || y == 0 || x == self.map[y].len() || y == self.map.len() {
            0
        } else {
            let cur_height = self.map[y][x];
            let mut dx_l = 1;
            while dx_l < x && cur_height > self.map[y][x - dx_l] {
                dx_l += 1;
            }

            let mut dx_r = 1;
            while dx_r + x < self.map[y].len()-1 && cur_height > self.map[y][x + dx_r] {
                dx_r += 1;
            }

            let mut dy_u = 1;
            while dy_u < y && cur_height > self.map[y - dy_u][x] {
                dy_u += 1;
            }

            let mut dy_d = 1;
            while dy_d + y < self.map.len()-1 && cur_height > self.map[y + dy_d][x] {
                dy_d += 1;
            }

            println!("{dx_l} * {dx_r} * {dy_u} * {dy_d}");
            dx_l * dx_r * dy_u * dy_d
        }
    }

    fn get_max_vis_score(&self) -> usize {
        let mut max_score: usize = usize::MIN;

        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[y].len() - 1 {
                let cur_score = self.get_vis_score(x, y);
                if max_score < cur_score {
                    println!("updating max score: {x} {y} = {}", self.map[y][x]);
                    max_score = cur_score;
                }
            }
        }

        max_score
    }
}

impl From<Vec<String>> for Map {
    fn from(input: Vec<String>) -> Self {
        let mut map: Vec<Vec<u8>> = Vec::new();

        input.iter().for_each(|line| {
            let mut row: Vec<u8> = Vec::new();
            line.chars().for_each(|tree| {
                row.push(match tree.to_digit(10) {
                    Some(digit) => digit as u8,
                    None => panic!("error converting digit {tree}"),
                })
            });
            map.push(row)
        });

        Map::new(map)
    }
}

fn task1(lines: Vec<String>) {
    let map = Map::from(lines);

    println!(
        "1. number of visible trees: {}",
        map.get_num_visible_trees()
    );
}

fn task2(lines: Vec<String>) {
    let map = Map::from(lines);

    println!(
        "2. maximum visibility score: {}",
        map.get_max_vis_score()
    );
}

fn main() {
    let file_name = PathBuf::from("./input/day8");

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
