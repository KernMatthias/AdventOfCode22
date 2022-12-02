use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

enum OppChoice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for OppChoice {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'A' => Ok(OppChoice::Rock),
            'B' => Ok(OppChoice::Paper),
            'C' => Ok(OppChoice::Scissors),
            _ => Err(format!("could not convert {ch} to opponents choice")),
        }
    }
}

enum Response {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Response {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'X' => Ok(Response::Rock),
            'Y' => Ok(Response::Paper),
            'Z' => Ok(Response::Scissors),
            _ => Err(format!("could not convert {ch} to response")),
        }
    }
}

enum ResponseV2 {
    Loose,
    Draw,
    Win,
}

impl TryFrom<char> for ResponseV2 {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'X' => Ok(ResponseV2::Loose),
            'Y' => Ok(ResponseV2::Draw),
            'Z' => Ok(ResponseV2::Win),
            _ => Err(format!("could not convert {ch} to response")),
        }
    }
}

fn main() {
    let filename = PathBuf::from("./input/day2");

    // part 1
    match read_lines(&filename) {
        Ok(lines) => {
            let score: u32 = lines
                .into_iter()
                .flatten()
                .map(move |line| {
                    let mut opts = line.chars();

                    let opp_choice: OppChoice = if let Some(opp_choice) = opts.next() {
                        match opp_choice.try_into() {
                            Ok(choice) => choice,
                            Err(e) => panic!("Invalid Input: {}", e),
                        }
                    } else {
                        panic!("could not take first");
                    };

                    let response: Response = if let Some(response) = opts.nth(1) {
                        match response.try_into() {
                            Ok(choice) => choice,
                            Err(e) => panic!("Invalid Input: {}", e),
                        }
                    } else {
                        panic!("could not interpret response");
                    };

                    // this sux, would change to something better
                    match (response, opp_choice) {
                        (Response::Rock, OppChoice::Rock) => 4_u32,
                        (Response::Rock, OppChoice::Paper) => 1_u32,
                        (Response::Rock, OppChoice::Scissors) => 7_u32,
                        (Response::Paper, OppChoice::Rock) => 8_u32,
                        (Response::Paper, OppChoice::Paper) => 5_u32,
                        (Response::Paper, OppChoice::Scissors) => 2_u32,
                        (Response::Scissors, OppChoice::Rock) => 3_u32,
                        (Response::Scissors, OppChoice::Paper) => 9_u32,
                        (Response::Scissors, OppChoice::Scissors) => 6_u32,
                    }
                })
                .sum();

            println!("1. accumulated score: {score}");
        }
        Err(e) => {
            panic!("Error reading the input file: {}", e);
        }
    };

    // part 2
    match read_lines(&filename) {
        Ok(lines) => {
            let score: u32 = lines
                .into_iter()
                .flatten()
                .map(move |line| {
                    let mut opts = line.chars();

                    let opp_choice: OppChoice = if let Some(opp_choice) = opts.next() {
                        match opp_choice.try_into() {
                            Ok(choice) => choice,
                            Err(e) => panic!("Invalid Input: {}", e),
                        }
                    } else {
                        panic!("could not take first");
                    };

                    let response: ResponseV2 = if let Some(response) = opts.nth(1) {
                        match response.try_into() {
                            Ok(choice) => choice,
                            Err(e) => panic!("Invalid Input: {}", e),
                        }
                    } else {
                        panic!("could not interpret response");
                    };

                    // this is not so good again
                    match (response, opp_choice) {
                        // 0 points for loose
                        (ResponseV2::Loose, OppChoice::Rock) => 3_u32, // scissors
                        (ResponseV2::Loose, OppChoice::Paper) => 1_u32, // rock
                        (ResponseV2::Loose, OppChoice::Scissors) => 2_u32, // paper
                        // 3 points for draw
                        (ResponseV2::Draw, OppChoice::Rock) => 4_u32, // rock
                        (ResponseV2::Draw, OppChoice::Paper) => 5_u32, // paper
                        (ResponseV2::Draw, OppChoice::Scissors) => 6_u32, // scissors
                        // 6 points for win
                        (ResponseV2::Win, OppChoice::Rock) => 8_u32, // paper
                        (ResponseV2::Win, OppChoice::Paper) => 9_u32, // scissors
                        (ResponseV2::Win, OppChoice::Scissors) => 7_u32, //rock
                    }
                })
                .sum();
            println!("score of 2nd part: {score}");
        }
        Err(e) => {
            panic!("Error reading the input file: {}", e);
        }
    };
}

fn read_lines(name: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(name)?;

    Ok(io::BufReader::new(file).lines())
}
