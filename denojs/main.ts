import { BufReader } from 'https://deno.land/std@0.170.0/io/buf_reader.ts';

const { args } = Deno;

class Config {
  constructor(
    public countLines: boolean,
    public countBytes: boolean,
    public countRunes: boolean
  ) {}
}

const getArgs = () => {
  return new Config(
    args.includes('-l'),
    args.includes('-b'),
    args.includes('-r')
  );
};

const countLines = async (reader: Deno.Reader): Promise<number> => {
  let counter = 0;
  const buffer = new Uint8Array(64);

  while (true) {
    const n = await reader.read(buffer);
    if (n === null) {
      break;
    }

    for (const b of buffer.slice(0, n)) {
      if (b === 10) {
        // if new line character
        counter++;
      }
    }
  }

  return counter;
};

const countBytes = async (reader: Deno.Reader): Promise<number> => {
  let counter = 0;
  while (true) {
    const n = await reader.read(new Uint8Array(1));
    if (n === null) {
      break;
    }
    counter++;
  }

  return counter;
};

const countRunes = async (reader: Deno.Reader): Promise<number> => {
  let counter = 0;
  const buffer = new Uint8Array(64);
  const decoder = new TextDecoder();

  while (true) {
    const n = await reader.read(buffer);
    if (n === null) {
      break;
    }

    counter += decoder.decode(buffer.slice(0, n), { stream: true }).length;
  }

  decoder.decode();

  return counter;
};

const countWords = async (reader: Deno.Reader): Promise<number> => {
    const buffer = new Uint8Array(1024);
    let wordCount = 0;
  
    while (true) {
      const n = await reader.read(buffer);
      if (n === null) {
        break;
      }
  
      const text = new TextDecoder().decode(buffer.slice(0, n));
      wordCount += text.split(/\s+/).length;
    }
  
    return wordCount;
};

export const count = (reader: Deno.Reader, config: Config) => {
  switch (true) {
    case config.countLines:
      return countLines(reader);
    case config.countBytes:
      return countBytes(reader);
    case config.countRunes:
      return countRunes(reader);
    default:
      return countWords(reader);
  }
};

async function run(config: Config): Promise<void> {
  // Read input from stdin
  const stdin = Deno.stdin;
  const reader = new BufReader(stdin);

  // Count the number of lines, bytes, runes, or words in the input
  const result = await count(reader, config);

  // Print the final count value
  console.log(result);
}

const main = () => {
  const args = getArgs();
  run(args);
};

// Parse the command-line arguments and execute the program
main();
