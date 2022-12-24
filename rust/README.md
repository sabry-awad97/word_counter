# Word Count

This code defines a command-line utility called wc, which stands for "word count." It allows the user to count lines, bytes, or runes in the input provided to the program. The input can come from standard input (stdin), which means the user can pipe the output of other commands into wc.

The get_args function uses the clap crate to parse command-line arguments. It defines three flags: -l, -b, and -r, which correspond to counting lines, bytes, and runes, respectively. If none of these flags are provided, the program will count the number of words in the input.

The count function takes an input reader and a Config struct as arguments and returns the count of lines, bytes, runes, or words in the input, depending on the values of the count_lines, count_bytes, and count_runes fields in the Config struct. The count_lines, count_bytes, and count_runes functions perform the actual counting by iterating over the input and incrementing a counter each time they encounter a new line, byte, or rune.

The run function reads input from stdin, calls the count function to count the lines, bytes, runes, or words, and then prints the result to the terminal.

The main function calls get_args to parse the command-line arguments, and then calls run to execute the program. If either of these functions return an error, main prints the error message to the terminal and exits the program with a nonzero exit code.
