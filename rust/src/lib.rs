use clap::{App, Arg};
use regex::Regex;
use std::{
    error::Error,
    io::{self, prelude::*, BufReader},
    str,
};

#[derive(Debug)]
// Define a public struct called Config
pub struct Config {
    pub is_lines: bool,
    pub is_bytes: bool,
    pub is_runes: bool,
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
        is_lines: matches.contains_id("lines"),
        is_bytes: matches.contains_id("bytes"),
        is_runes: matches.contains_id("runes"),
    })
}

fn count_lines<R: Read>(reader: &mut R) -> Result<usize, Box<dyn Error>> {
    let reader = BufReader::new(reader);
    Ok(reader.lines().count())
}

fn count_bytes<R: Read>(reader: &mut R) -> Result<usize, Box<dyn Error>> {
    Ok(reader.bytes().count())
}

fn count_runes<R: Read>(reader: &mut R) -> Result<usize, str::Utf8Error> {
    let mut count = 0;
    let mut buffer = [0; 1024];
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        let s = str::from_utf8(&buffer[..n])?;
        count += s.chars().count();
    }
    Ok(count)
}

pub fn count_words<R: Read>(reader: &mut R) -> Result<usize, Box<dyn Error>> {
    let mut buffer = String::with_capacity(1024);
    reader.read_to_string(&mut buffer)?;
    let re = Regex::new(r"\b\S+\b")?;
    Ok(re.find_iter(&buffer).count())
}

pub fn count<R: Read>(reader: &mut R, config: Config) -> Result<usize, Box<dyn Error>> {
    let Config {
        is_lines,
        is_bytes,
        is_runes,
    } = config;

    match (is_lines, is_bytes, is_runes) {
        (true, false, false) => Ok(count_lines(reader)?),
        (false, true, false) => Ok(count_bytes(reader)?),
        (false, false, true) => Ok(count_runes(reader)?),
        (false, false, false) => Ok(count_words(reader)?),
        _ => Err(From::from("Invalid combination of flags")),
    }
}

pub fn run(config: Config) -> MainResult<()> {
    // Read input from stdin
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Count the number of lines, bytes, runes, or words in the input
    let count = count(&mut reader, config)?;

    // Print the final count value
    println!("{}", count);

    Ok(())
}
