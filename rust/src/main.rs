fn main() {
    if let Err(e) = word_counter::get_args().and_then(word_counter::run) {
        // error print line
        eprintln!("{}", e);

        // Exit the program with a nonzero value to indicate an error.
        std::process::exit(1);
    }
}

// echo "This is a test" | cargo run
// echo "This is a test" | cargo run -l
