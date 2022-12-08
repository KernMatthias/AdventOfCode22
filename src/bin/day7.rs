use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::PathBuf;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    parent: u32,
    entries: Vec<u32>,
}

impl Directory {
    fn new(name: String, parent: u32, entries: Vec<u32>) -> Self {
        Self {
            name,
            parent,
            entries,
        }
    }
}

#[derive(Debug)]
struct Fil {
    name: String,
    size: u32,
}

impl Fil {
    fn new(name: String, size: u32) -> Self {
        Self { name, size }
    }
}

#[derive(Debug)]
enum FsEntry {
    Dir(Directory),
    File(Fil),
}

impl FsEntry {
    fn calc_size(&self, fs: &FileSystem) -> u32 {
        match self {
            Self::Dir(dir) => dir
                .entries
                .iter()
                .map(|entry| fs.nodes[*entry as usize].calc_size(fs))
                .sum::<u32>(),
            Self::File(file) => file.size,
        }
    }

    fn fmt(
        &self,
        fs: &FileSystem,
        f: &mut std::fmt::Formatter<'_>,
        level: u32,
    ) -> std::fmt::Result {
        let indent = " |".repeat(level as usize);
        match self {
            Self::Dir(dir) => {
                write!(f, "{}\\_{} - {}\n", indent, dir.name, self.calc_size(fs))?;
                for entry in &dir.entries {
                    fs.nodes[*entry as usize].fmt(fs, f, level + 1)?;
                }
                Ok(())
            }
            Self::File(file) => write!(f, "{}{} - {}\n", indent, file.name, file.size),
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<FsEntry>,
}

impl FileSystem {
    fn new(nodes: Vec<FsEntry>) -> Self {
        Self { nodes }
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nodes[0].fmt(&self, f, 0)
    }
}

impl From<Vec<Command>> for FileSystem {
    fn from(commands: Vec<Command>) -> Self {
        Self::from_iter(commands.into_iter())
    }
}

impl FromIterator<Command> for FileSystem {
    fn from_iter<T: IntoIterator<Item = Command>>(iter: T) -> Self {
        let mut nodes: Vec<FsEntry> = Vec::new();
        let mut index = 0_u32;

        nodes.push(FsEntry::Dir(Directory::new("/".to_string(), 0, Vec::new())));
        index += 1;

        let mut cur_dir = 0_u32;

        for cmd in iter {
            match cmd {
                Command::CD(name) => {
                    cur_dir = if let FsEntry::Dir(cur_entry) = &nodes[cur_dir as usize] {
                        if name == "/" {
                            0_u32
                        } else if name == ".." {
                            cur_entry.parent
                        } else {
                            let mut new_dir = -1_i32;

                            for entry in &cur_entry.entries {
                                if let FsEntry::Dir(cur_entry) = &nodes[*entry as usize] {
                                    if cur_entry.name == name {
                                        new_dir = *entry as i32;
                                        break;
                                    }
                                }
                            }
                            if new_dir == -1 {
                                panic!("did not find directory {} to switch to...", name);
                            }
                            new_dir as u32
                        }
                    } else {
                        panic!("cur dir is no dir?")
                    }
                }
                Command::LS(content) => {
                    for entry in content {
                        if entry.starts_with("dir ") {
                            let mut entry = entry.split_whitespace();
                            nodes.push(FsEntry::Dir(Directory::new(
                                entry.nth(1).expect("no name?").to_string(),
                                cur_dir,
                                Vec::new(),
                            )));
                        } else {
                            let entries = entry.split_whitespace().collect_vec();
                            let size = entries[0]
                                .parse::<u32>()
                                .expect("invalid input, could not parse to int");

                            nodes.push(FsEntry::File(Fil::new(entries[1].to_string(), size)));
                        }

                        let cur = if let FsEntry::Dir(cur) = &mut nodes[cur_dir as usize] {
                            cur
                        } else {
                            panic!("")
                        };
                        cur.entries.push(index);
                        index += 1;
                    }
                }
            }
        }

        FileSystem::new(nodes)
    }
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS(Vec<String>),
}

impl From<String> for Command {
    fn from(command: String) -> Self {
        let command_vec = command
            .split('\n')
            .flat_map(|line| if line.is_empty() { None } else { Some(line) })
            .collect_vec();

        let mut command = command_vec.into_iter();

        let mut call = command.next().unwrap().split_whitespace();

        match call.nth(0).expect("invalid input") {
            "cd" => Command::CD(call.next().expect("").to_string()),
            "ls" => Command::LS(command.map(|line| line.to_string()).collect_vec()),
            a => panic!("invalid input: {}", a),
        }
    }
}

fn task1(lines: Vec<String>) {
    let commands = lines
        .join("\n")
        .split(|char| char == '$')
        .flat_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.chars().skip(1).collect::<String>()) // skip the space after $
            }
        })
        .map(|group| Command::from(group))
        .collect_vec();

    let fs = FileSystem::from(commands);

    println!("{}", fs);

    let acc_size = fs
        .nodes
        .iter()
        .flat_map(|entry| match entry {
            FsEntry::Dir(_) => Some(entry.calc_size(&fs)),
            FsEntry::File(_) => None,
        })
        .filter(|entry| *entry <= 100000)
        .sum::<u32>();

    println!(
        "1. accumulated size of all directories with less than 100000 size: {}",
        acc_size
    );
}

fn task2(lines: Vec<String>) {
    let commands = lines
        .join("\n")
        .split(|char| char == '$')
        .flat_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.chars().skip(1).collect::<String>()) // skip the space after $
            }
        })
        .map(|group| Command::from(group))
        .collect_vec();

    let fs = FileSystem::from(commands);

    let fs_size: u32 = fs
        .nodes
        .iter()
        .flat_map(|entry| match entry {
            FsEntry::Dir(_) => None,
            FsEntry::File(_) => Some(entry.calc_size(&fs)),
        })
        .sum();
    println!("size of the whole fs: {}", fs_size);

    let needed_size = fs
        .nodes
        .iter()
        .flat_map(|entry| match entry {
            FsEntry::Dir(_) => Some(entry.calc_size(&fs)),
            FsEntry::File(_) => None,
        })
        .sorted()
        .find(move |size| *size >= fs_size - (70_000_000 - 30_000_000))
        .expect("not found");

    println!("2. size of the directory to remove: {}", needed_size);
}

fn main() {
    let file_name = PathBuf::from("./input/day7");

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
