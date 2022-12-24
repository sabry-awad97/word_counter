#[test]
fn test_count_lines() {
    let input = "Line 1\nLine 2\nLine 3";
    let config = word_counter::Config {
        count_lines: true,
        count_bytes: false,
        count_runes: false,
    };

    assert_eq!(word_counter::count(input, config), 3);
}

#[test]
fn test_count_bytes() {
    let input = "Hello, world!";
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: true,
        count_runes: false,
    };
    assert_eq!(word_counter::count(input, config), 13);
}

#[test]
fn test_count_runes() {
    let input = "Hello, 世界！";
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: false,
        count_runes: true,
    };
    assert_eq!(word_counter::count(input, config), 10);
}

#[test]
fn test_count_words() {
    let input = "This is a test";
    let config = word_counter::Config {
        count_lines: false,
        count_bytes: false,
        count_runes: false,
    };
    assert_eq!(word_counter::count(input, config), 4);
}
