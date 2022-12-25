import { assertEquals } from 'https://deno.land/std@0.170.0/testing/asserts.ts';
import { Buffer } from 'https://deno.land/std@0.170.0/io/buffer.ts';
import { BufReader } from 'https://deno.land/std@0.170.0/io/buf_reader.ts';
import { count } from './main.ts';

Deno.test('count lines in input text', async () => {
  // Arrange
  const input = 'line 1\nline 2\nline 3\n';
  const config = { countLines: true, countBytes: false, countRunes: false };
  const expected = 3;

  // Act
  const buffer = new Buffer(new TextEncoder().encode(input));
  const reader = new BufReader(buffer);
  const result = await count(reader, config);

  // Assert
  assertEquals(result, expected);
});

Deno.test('count bytes in input text', async () => {
  // Arrange
  const input = 'Hello 世界!';
  const config = { countLines: false, countBytes: true, countRunes: false };
  const expected = 13;

  // Act
  const buffer = new Buffer(new TextEncoder().encode(input));
  const reader = new BufReader(buffer);
  const result = await count(reader, config);

  // Assert
  assertEquals(result, expected);
});

Deno.test('count chars in input text', async () => {
  // Arrange
  const input = 'Hello 世界!';
  const config = { countLines: false, countBytes: false, countRunes: true };
  const expected = 9;

  // Act
  const buffer = new Buffer(new TextEncoder().encode(input));
  const reader = new BufReader(buffer);
  const result = await count(reader, config);

  // Assert
  assertEquals(result, expected);
});

Deno.test('count words in input text', async () => {
  // Arrange
  const input = 'Hello 世界!';
  const config = { countLines: false, countBytes: false, countRunes: false };
  const expected = 2;

  // Act
  const buffer = new Buffer(new TextEncoder().encode(input));
  const reader = new BufReader(buffer);
  const result = await count(reader, config);

  // Assert
  assertEquals(result, expected);
});
