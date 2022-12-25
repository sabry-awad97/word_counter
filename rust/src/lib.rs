use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
// Define a public struct called Config
pub struct Config {
    pub count_lines: bool,
    pub count_bytes: bool,
    pub count_runes: bool,
}

// Create a Result to represent an Ok value for any type T or some Err value that implements the Error trait
type MainResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MainResult<Config> {
    let matches = App::new("wc")
        .arg(Arg::new("lines").short('l').help("Count lines"))
        .arg(Arg::new("bytes").short('b').help("Count bytes"))
        .arg(Arg::new("runes").short('r').help("Count runes"))
        .get_matches();

    Ok(Config {
        count_lines: matches.contains_id("lines"),
        count_bytes: matches.contains_id("bytes"),
        count_runes: matches.contains_id("runes"),
    })
}

fn count_lines<R: Read>(reader: &mut R) -> usize {
    let reader = BufReader::new(reader);
    reader.lines().count()
}

fn count_bytes<R: Read>(reader: &mut R) -> usize {
    reader.bytes().count()
}

fn count_runes<R: Read>(reader: &mut R) -> usize {
    let mut reader = BufReader::new(reader);
    let mut count = 0;
    let mut buffer = Vec::new();
    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        let line = String::from_utf8(buffer.clone()).unwrap();
        count += line.chars().count();
        buffer.clear();
    }
    count
}

pub fn count_words<R: Read>(reader: &mut R) -> usize {
    let mut buffer = String::with_capacity(1024);
    reader.read_to_string(&mut buffer).unwrap();
    // matches any sequence of characters surrounded by word boundaries
    let re = Regex::new(r"\b\S+\b").unwrap();
    re.find_iter(&buffer).count()
}

pub fn count<R: Read>(reader: &mut R, config: Config) -> usize {
    match (config.count_lines, config.count_bytes, config.count_runes) {
        (true, false, false) => count_lines(reader),
        (false, true, false) => count_bytes(reader),
        (false, false, true) => count_runes(reader),
        (false, false, false) => count_words(reader),
        _ => panic!("Invalid combination of flags"),
    }
}

pub fn run(config: Config) -> MainResult<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Count the number of lines, bytes, runes, or words in the input
    let count = count(&mut reader, config);

    // Print the final count value
    println!("{}", count);

    Ok(())
}
