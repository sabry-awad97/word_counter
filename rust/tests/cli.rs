#[test]
fn test_count_lines() {
    let input = "Line 1\nLine 2\nLine 3";
    assert_eq!(word_counter::count(input, true, false, false), 3);
}

#[test]
fn test_count_bytes() {
    let input = "Hello, world!";
    assert_eq!(word_counter::count(input, false, true, false), 13);
}

#[test]
fn test_count_runes() {
    let input = "Hello, 世界！";
    assert_eq!(word_counter::count(input, false, false, true), 10);
}

#[test]
fn test_count_words() {
    let input = "This is a test";
    assert_eq!(word_counter::count(input, false, false, false), 4);
}
