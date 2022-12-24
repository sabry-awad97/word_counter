# Word Count

This Rust program is a command line utility that counts lines, bytes, runes, or words in the input it receives from stdin. It uses the clap crate to parse command line arguments passed to it at runtime and determine what to count.

The get_args function uses the clap crate to define command line arguments for the program. It defines three arguments: lines, bytes, and runes. Each argument is defined with a short version, specified with the short method, and a help message, specified with the help method. The get_matches method parses the command line arguments and returns a Matches object that contains information about the arguments that were passed to the program.

The Config struct holds three boolean fields, count_lines, count_bytes, and count_runes, which indicate whether or not the corresponding command line arguments were passed to the program. The get_args function creates and returns a Config object based on the values of the command line arguments contained in the Matches object.

The count function takes a mutable reference to an io::Read trait object and a Config object as arguments and returns a usize value. It uses a pattern match to determine which count function to call based on the values of the fields in the Config object. If the count_lines field is true, it calls the count_lines function; if the count_bytes field is true, it calls the count_bytes function; if the count_runes field is true, it calls the count_runes function; and if none of these fields are true, it calls the count_words function.

The run function reads input from stdin using a BufRead trait object, and passes this input to the count function along with the Config object. It then prints the result returned by the count function to the console.

Finally, the main function calls the get_args function to parse the command line arguments and create a Config object. It then calls the run function, passing it the Config object. If either function returns an error, it prints the error message to stderr and exits the program with a nonzero value to indicate an error. If both functions succeed, the program ends normally.
