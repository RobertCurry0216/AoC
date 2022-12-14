const fs = require("fs");

const buildData = (filePath) => {
  const data = fs
    .readFileSync(filePath, "utf8")
    .split(/\n/)
    .map((l) => l.split(" -> "));
  const map = new Set();
  let maxY = 0;

  for (const line of data) {
    let [x, y] = line.shift().split(",");
    x = +x;
    y = +y;
    map.add(`${x}-${y}`);

    for (const segment of line) {
      let [dx, dy] = segment.split(",");
      dx = +dx;
      dy = +dy;
      const steps = Math.max(Math.abs(dx - x), Math.abs(dy - y));
      dx = Math.sign(dx - x);
      dy = Math.sign(dy - y);

      for (let i = 0; i < steps; i++) {
        x += dx;
        y += dy;
        map.add(`${x}-${y}`);
        maxY = Math.max(maxY, y);
      }
    }
  }

  return [map, maxY];
};

const canAddGrainOfSand = (map, maxY) => {
  let x = 500;
  let y = 0;
  while (y < maxY) {
    if (!map.has(`${x}-${y + 1}`)) {
      y += 1;
    } else if (!map.has(`${x - 1}-${y + 1}`)) {
      x -= 1;
      y += 1;
    } else if (!map.has(`${x + 1}-${y + 1}`)) {
      x += 1;
      y += 1;
    } else {
      map.add(`${x}-${y}`);
      return true;
    }
  }
  return false;
};

const part1 = (filePath) => {
  const [map, maxY] = buildData(filePath);
  let grains = 0;

  while (grains < 10_000) {
    if (!canAddGrainOfSand(map, maxY)) break;
    grains += 1;
  }

  return grains;
};

const canAddGrainOfSandWithFloor = (map, maxY) => {
  let x = 500;
  let y = 0;
  while (y < maxY + 1) {
    if (!map.has(`${x}-${y + 1}`)) {
      y += 1;
    } else if (!map.has(`${x - 1}-${y + 1}`)) {
      x -= 1;
      y += 1;
    } else if (!map.has(`${x + 1}-${y + 1}`)) {
      x += 1;
      y += 1;
    } else {
      break
    }
  }
  map.add(`${x}-${y}`);
  return !(x == 500 && y == 0);
};

const part2 = (filePath) => {
  const [map, maxY] = buildData(filePath);
  let grains = 0;

  while (grains < 100_000) {
    grains += 1;
    if (!canAddGrainOfSandWithFloor(map, maxY)) break;
  }

  return grains;
};

const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`);
  console.log(`part 2: ${part2("./input.txt")}`);
};

main();
