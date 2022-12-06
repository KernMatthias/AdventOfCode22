use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::PathBuf;

fn char_windows<'a>(src: &'a str, win_size: usize) -> impl Iterator<Item = &'a str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() {
    let file_name = PathBuf::from("./input/day6");

    // task 1
    match read_lines(&file_name) {
        Ok(mut lines) => {
            let line = lines.next().expect("invalid input").expect("invalid input");
            let windows = char_windows(&line, 4);

            let (cnt, _) = windows
                .map(|w| has_unique_elements(w.chars()))
                .enumerate()
                .skip_while(|(_, b)| !b)
                .next()
                .expect("nothing unique...");

            println!("1. first start-of-packet after {}", cnt + 4);
        }
        Err(e) => println!("Error: {}", e),
    }

    // task 2
    match read_lines(&file_name) {
        Ok(mut lines) => {
            let line = lines.next().expect("invalid input").expect("invalid input");
            let windows = char_windows(&line, 14);

            let (cnt, _) = windows
                .map(|w| has_unique_elements(w.chars()))
                .enumerate()
                .skip_while(|(_, b)| !b)
                .next()
                .expect("nothing unique...");

            println!("1. first start-of-message after {}", cnt + 14);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
