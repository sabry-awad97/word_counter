import readline from 'readline';

// Define the count function to count the number of words, lines, bytes, or runes
// in the given input
const count = (
  input: NodeJS.ReadableStream,
  isLines?: boolean,
  isBytes?: boolean,
  isRunes?: boolean
) => {
  let counter = 0;

  // Create a readline interface to iterate through the input
  const rl = readline.createInterface({ input });

  // Define the line splitting function based on the flags provided
  let lineSplitFn: (line: string) => string[];

  switch (true) {
    case isLines:
      lineSplitFn = (line: string) => [line];
      break;
    case isBytes:
      lineSplitFn = (line: string) => line.split('');
      break;
    case isRunes:
      lineSplitFn = (line: string) => Array.from(line);
      break;

    default:
      lineSplitFn = (line: string) => line.trim().split(/\s+/);
  }

  // For every line in the input, split it into words, lines, bytes, or runes
  // and increment the counter
  rl.on('line', line => {
    counter += lineSplitFn(line).length;
  });

  // Return a promise that resolves with the final counter value
  return new Promise<number>(r => rl.on('close', () => r(counter)));
};

// Parse the flags provided by the user
const flags = process.argv.slice(2);

// The normal behavior is to count words
let countLines = false;
let countBytes = false;
let countRunes = false;

// Check for the -l, -b, and -r flags
if (flags.includes('-l')) {
  countLines = true;
}
if (flags.includes('-b')) {
  countBytes = true;
}
if (flags.includes('-r')) {
  countRunes = true;
}

// Call the count function to count the number of words, lines, bytes, or runes
// received from the Standard Input (or a file, if provided as an argument)
// and print the result
count(process.stdin, countLines, countBytes, countRunes).then(count => {
  console.log(count);
});
