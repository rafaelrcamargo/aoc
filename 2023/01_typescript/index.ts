import { readFile } from "fs/promises";

const N: Record<string, number> = {
  one: 1,
  two: 2,
  three: 3,
  four: 4,
  five: 5,
  six: 6,
  seven: 7,
  eight: 8,
  nine: 9
};

const toNumber = (line: string, spelled = false) => {
  const acc = [];

  for (let i = 0; i < line.length; i++) {
    if (spelled) {
      const num = isNaN(Number(line[i]))
        ? N[line.substring(i, i + 3)] ??
          N[line.substring(i, i + 4)] ??
          N[line.substring(i, i + 5)]
        : Number(line[i]);

      num && acc.push(String(num));
    } else {
      const num = !isNaN(Number(line[i])) && Number(line[i]);

      num && acc.push(String(num));
    }
  }

  return acc;
};

const input = await readFile("./assets/input.txt", "utf-8");
const lines = input.split("\n");

console.log(
  "Part One:",
  lines
    .map((line) => toNumber(line).filter(Boolean))
    .filter((arr) => arr.length > 0)
    .map((row) => row.at(0)! + row.at(-1))
    .map((num) => Number(num))
    .reduce((a, b) => a + b, 0)
);

console.log(
  "Part Two:",
  lines
    .map((line) => toNumber(line, true).filter(Boolean))
    .filter((arr) => arr.length > 0)
    .map((row) => row.at(0)! + row.at(-1))
    .map((num) => Number(num))
    .reduce((a, b) => a + b, 0)
);
