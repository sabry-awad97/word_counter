# Word Count

This Rust program is a command line utility that counts lines, bytes, runes, or words in the input it receives from stdin. It uses the clap crate to parse command line arguments passed to it at runtime and determine what to count.

The get_args function uses the clap crate to parse command line arguments. It creates a new App object and adds three arguments with the short names l, b, and r for the lines, bytes, and runes options, respectively. The get_matches method is called to parse the command line arguments and return a Matches object. The is_lines, is_bytes, and is_runes fields of the returned Config struct are set to true if the corresponding command line argument is present, and false otherwise.

The count_lines, count_bytes, count_runes, and count_words functions are used to count the number of lines, bytes, runes, or words in a Read type object, respectively. The count function takes a Read object and a Config object as arguments and returns the count based on the values of the is_lines, is_bytes, and is_runes fields in the Config object.

The run function reads input from standard input and calls the count function to count the number of lines, bytes, runes, or words based on the provided Config object. Finally, the main function calls get_args to parse the command line arguments, calls run with the returned Config object, and handles any errors that may occur.
