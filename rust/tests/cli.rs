use std::{
    error::Error,
    io::{BufReader, Cursor},
};

use word_counter::{count, Config};

fn create_buf_reader(input: &str) -> Result<BufReader<Cursor<Vec<u8>>>, Box<dyn Error>> {
    let cursor = Cursor::new(input.as_bytes().to_vec());
    Ok(BufReader::new(cursor))
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_count_lines() -> TestResult {
    // Arrange
    let input = "line 1\nline 2\nline 3\n";
    let expected = 3;
    let config = Config {
        is_lines: true,
        is_bytes: false,
        is_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned())?;
    let count = count(&mut reader, config)?;

    // Assert
    assert_eq!(count, expected);
    Ok(())
}

#[test]
fn test_count_bytes() -> TestResult {
    let input = "Hello 世界!'";
    let expected = 14;
    let config = Config {
        is_lines: false,
        is_bytes: true,
        is_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned())?;
    let count = count(&mut reader, config)?;

    // Assert
    assert_eq!(count, expected);
    Ok(())
}

#[test]
fn test_count_runes() -> TestResult {
    let input = "Hello 世界!'";
    let expected = 10;
    let config = Config {
        is_lines: false,
        is_bytes: false,
        is_runes: true,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned())?;
    let count = count(&mut reader, config)?;

    // Assert
    assert_eq!(count, expected);
    Ok(())
}

#[test]
fn test_count_words() -> TestResult {
    let input = "hello world how are you";
    let expected = 5;
    let config = Config {
        is_lines: false,
        is_bytes: false,
        is_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned())?;
    let count = count(&mut reader, config)?;

    // Assert
    assert_eq!(count, expected);
    Ok(())
}
