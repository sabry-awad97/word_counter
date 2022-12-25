use std::io::{BufReader, Cursor};

fn create_buf_reader(input: &str) -> BufReader<Cursor<Vec<u8>>> {
    let cursor = Cursor::new(input.as_bytes().to_vec());
    BufReader::new(cursor)
}

#[test]
fn test_count_lines() {
    // Arrange
    let input = "line 1\nline 2\nline 3\n";
    let expected = 3;
    let config = word_counter::Config {
        count_lines: true,
        count_bytes: false,
        count_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned());
    let count = word_counter::count(&mut reader, config);

    // Assert
    assert_eq!(count, expected);
}

#[test]
fn test_count_bytes() {
    let input = "Hello 世界!'";
    let expected = 14;
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: true,
        count_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned());
    let count = word_counter::count(&mut reader, config);

    // Assert
    assert_eq!(count, expected);
}

#[test]
fn test_count_runes() {
    let input = "Hello 世界!'";
    let expected = 10;
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: false,
        count_runes: true,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned());
    let count = word_counter::count(&mut reader, config);

    // Assert
    assert_eq!(count, expected);
}

#[test]
fn test_count_words() {
    let input = "hello world how are you";
    let expected = 5;
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: false,
        count_runes: false,
    };

    // Act
    let mut reader = create_buf_reader(&input.to_owned());
    let count = word_counter::count(&mut reader, config);

    // Assert
    assert_eq!(count, expected);
}
