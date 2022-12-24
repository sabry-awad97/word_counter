import * as readline from 'readline';

import { Command } from 'commander';

class Config {
  constructor(
    public countLines: boolean,
    public countBytes: boolean,
    public countRunes: boolean
  ) {}
}

// Parse the command-line arguments and return a Config object
function getArgs(): Config {
  const program = new Command('wc');
  program
    .option('-l, --lines', 'Count lines')
    .option('-b, --bytes', 'Count bytes')
    .option('-r, --runes', 'Count runes')
    .parse(process.argv);

  const args = program.opts<{
    lines?: boolean;
    bytes?: boolean;
    runes?: boolean;
  }>();

  return new Config(
    args.lines || false,
    args.bytes || false,
    args.runes || false
  );
}

// Count the number of lines in the input
function countLines(input: string): number {
  return input.split('\n').length - 1;
}

// Count the number of bytes in the input
function countBytes(input: string): number {
  return Buffer.byteLength(input);
}

// Count the number of runes in the input
function countRunes(input: string): number {
  return input.length;
}

// Count the number of words in the input
function countWords(input: string): number {
  return input.split(/\s+/).length;
}

// Count the lines, bytes, runes, or words in the input
function count(input: string, config: Config): number {
  if (config.countLines) {
    return countLines(input);
  } else if (config.countBytes) {
    return countBytes(input);
  } else if (config.countRunes) {
    return countRunes(input);
  } else {
    return countWords(input);
  }
}

// Read input from stdin and print the result
function run(config: Config): void {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
  });

  let input = '';
  rl.on('line', (line: string) => {
    input += line + '\n';    
  });
  
  rl.on('close', () => {
    console.log(count(input, config));
  });

  rl.on('error', (err: any) => {
    console.log(err);
  });
}

// Parse the command-line arguments and execute the program
const args = getArgs();
run(args);
