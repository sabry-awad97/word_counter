import argparse
import sys

# Define the count function to count the number of words, lines, bytes, or characters
# in the given input


def count(input, count_lines=False, count_bytes=False, count_chars=False):
    counter = 0

    # Iterate through the input and split it into words, lines, bytes, or characters
    # based on the flags provided
    for line in input:
        if count_lines:
            counter += 1
        elif count_bytes:
            counter += len(line.encode('utf-8'))
        elif count_chars:
            counter += len(line)
        else:
            counter += len(line.split())

    return counter


# Create the argument parser
parser = argparse.ArgumentParser()

# Add the -l, -b, and -c flags
parser.add_argument('-l', action='store_true', help='count lines')
parser.add_argument('-b', action='store_true', help='count bytes')
parser.add_argument('-c', action='store_true', help='count characters')

# Add the file argument
parser.add_argument('file', nargs='?', help='input file')

# Parse the arguments
args = parser.parse_args()

# The normal behavior is to count words
count_lines = False
count_bytes = False
count_chars = False

# Check for the -l, -b, and -c flags
if args.l:
    count_lines = True
if args.b:
    count_bytes = True
if args.c:
    count_chars = True

# Call the count function to count the number of words, lines, bytes, or characters
# received from the Standard Input (or a file, if provided as an argument)
# and print the result
if args.file:
    with open(args.file, 'r') as f:
        print(count(f, count_lines, count_bytes, count_chars))
else:
    print(count(sys.stdin, count_lines, count_bytes, count_chars))
