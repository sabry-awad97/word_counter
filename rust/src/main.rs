use word_counter::{get_args, run};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        // error print line
        eprintln!("{}", e);

        // Exit the program with a nonzero value to indicate an error.
        std::process::exit(1);
    }
}

// echo "This is a test" | cargo run
// echo "This is a test" | cargo run -l
