import argparse
import sys


def count_lines(reader):
    counter = 0
    for _ in reader:
        counter += 1
    return counter


def count_bytes(reader):
    return len(reader.read())


def count_runes(reader):
    counter = 0
    for line in reader:
        counter += len(line)
    return counter


def count_words(reader):
    counter = 0
    for line in reader:
        counter += len(line.split())
    return counter


def count(reader, c_lines, c_bytes, c_runes):
    if c_lines:
        return count_lines(reader)

    if c_bytes:
        return count_bytes(reader)

    if c_runes:
        return count_runes(reader)

    return count_words(reader)


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-l", "--lines", action="store_true", help="Count lines")
    parser.add_argument(
        "-b", "--bytes", action="store_true", help="Count bytes")
    parser.add_argument(
        "-r", "--runes", action="store_true", help="Count runes")
    args = parser.parse_args()

    return args


def main():
    args = get_args()
    reader = sys.stdin
    result = count(reader, args.lines, args.bytes, args.runes)
    print(result)


if __name__ == "__main__":
    main()
