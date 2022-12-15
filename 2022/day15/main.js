const fs = require("fs");

re =
  /Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)/;

const buildData = (filePath) => {
  const lines = fs.readFileSync(filePath, "utf8").split(/\n/);
  const data = [];

  for (const line of lines) {
    const match = line.match(re);
    data.push({
      sx: +match[1],
      sy: +match[2],
      bx: +match[3],
      by: +match[4],
    });
  }

  return data;
};

const manhattan = (x, y, a, b) => Math.abs(x - a) + Math.abs(y - b);

const part1 = (filePath, target) => {
  const data = buildData(filePath);
  const targetSlices = [];

  for (const v of data) {
    const dist = manhattan(v.sx, v.sy, v.bx, v.by);
    if (v.sy + dist >= target && v.sy - dist <= target) {
      const diff = dist - Math.abs(target - v.sy);
      targetSlices.push([v.sx - diff, v.sx + diff]);
    }
  }

  targetSlices.sort((a, b) => a[0] - b[0]);

  let slice = targetSlices[0];
  let total = 0;

  for (let i = 1; i < targetSlices.length; i++) {
    const next = targetSlices[i];
    if (slice[1] >= next[0]) {
      // overlap
      slice[1] = Math.max(slice[1], next[1]);
    } else {
      // no overlap
      total += slice[1] - slice[0];
    }
  }

  total += slice[1] - slice[0];

  return total;
};

const part2 = (filePath, size) => {
  const data = buildData(filePath);

  for (let y = 0; y < size; y++) {
    const ySlices = data
      .map((v) => {
        const dist = manhattan(v.sx, v.sy, v.bx, v.by);
        if (v.sy + dist >= y && v.sy - dist <= y) {
          const diff = dist - Math.abs(y - v.sy);
          return [v.sx - diff, v.sx + diff];
        }
      })
      .filter((v) => v)
      .map(([a, b]) => [Math.max(a, 0), Math.min(b, size)]);

    ySlices.sort((a, b) => a[0] - b[0]);

    let slice = ySlices[0];

    for (let i = 1; i < ySlices.length; i++) {
      const next = ySlices[i];
      if (slice[1] >= next[0]) {
        // overlap
        slice[1] = Math.max(slice[1], next[1]);
      } else {
        // no overlap
        return (slice[1] + 1) * 4000000 + y;
      }
    }
  }
};

const main = () => {
  console.log(`part 1: ${part1("./input.txt", 2000000)}`);
  console.log(`part 2: ${part2("./input.txt", 4000000)}`);
};

main();
