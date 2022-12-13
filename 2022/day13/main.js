const fs = require("fs");

const parseLine = (line) => JSON.parse(line);

const buildInput = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const out = [];

  while (data.length > 0) {
    const left = parseLine(data.shift());
    const right = parseLine(data.shift());
    data.shift(); //empty line
    out.push([left, right])
  }

  return out;
}

const compare = (left, right) => {
  if (typeof left == typeof right && typeof left == "number") return left - right;

  if (!Array.isArray(left)) return compare([left], right);
  if (!Array.isArray(right)) return compare(left, [right]);

  const steps = Math.min(left.length, right.length);

  for (let i = 0; i<steps; i++) {
    const c = compare(left[i], right[i]);
    if (c != 0) return c;
  }

  return left.length - right.length;
}

const part1 = (filePath) => {
  const data = buildInput(filePath);

  return data.reduce((acc, cur, idx) => {
    if (compare(cur[0], cur[1]) < 0) {
      return acc + idx + 1;
    }
    return acc;
  }, 0)
}


const part2 = (filePath) => {
  const data = buildInput(filePath).flat();
  const divider2 = [[2]];
  const divider6 = [[6]];
  data.push(divider2);
  data.push(divider6);
  data.sort(compare);

  const idx2 = data.findIndex(v => v == divider2) + 1;
  const idx6 = data.findIndex(v => v == divider6) + 1;
  return idx2 * idx6;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()