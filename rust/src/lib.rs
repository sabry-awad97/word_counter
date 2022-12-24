use std::env;
use std::error::Error;
use std::io;

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
    let mut count_lines = false;
    let mut count_bytes = false;
    let mut count_runes = false;

    // Parse the flags provided by the user
    let mut args = env::args();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" => count_lines = true,
            "-b" => count_bytes = true,
            "-r" => count_runes = true,
            _ => {}
        }
    }

    Ok(Config {
        count_lines,
        count_bytes,
        count_runes,
    })
}

pub fn count(input: &str, config: Config) -> usize {
    match (config.count_lines, config.count_bytes, config.count_runes) {
        (true, false, false) => input.lines().count(),
        (false, true, false) => input.bytes().count(),
        (false, false, true) => input.chars().count(),
        (false, false, false) => input.split_whitespace().count(),
        _ => panic!("Invalid combination of flags"),
    }
}

pub fn run(config: Config) -> MainResult<()> {
    // Read input from stdin
    let mut input = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut input).unwrap();

    // Count the number of lines, bytes, runes, or words in the input
    let count = count(&input, config);

    // Print the final count value
    println!("{}", count);

    Ok(())
}
