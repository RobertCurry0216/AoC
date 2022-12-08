const fs = require("fs");

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const forrest = data.map(line => line.split("").map(v => +v));
  const h = forrest.length;
  const w = forrest[0].length;

  const visible = new Set();

  //left
  for (let y=0; y<h; y++) {
    let highest = -1;
    for (let x=0; x<w; x++) {
      if (forrest[y][x] > highest) {
        visible.add(`${x}-${y}`);
        highest = forrest[y][x];
      }
    }
  }

  //right
  for (let y=h-1; y>=0; y--) {
    let highest = -1;
    for (let x=w-1; x>=0; x--) {
      if (forrest[y][x] > highest) {
        visible.add(`${x}-${y}`);
        highest = forrest[y][x];
      }
    }
  }

  //top
  for (let x=0; x<w; x++) {
    let highest = -1;
    for (let y=0; y<h; y++) {
      if (forrest[y][x] > highest) {
        visible.add(`${x}-${y}`);
        highest = forrest[y][x];
      }
    }
  }

  //bottom
  for (let x=w-1; x>=0; x--) {
    let highest = -1;
    for (let y=h-1; y>=0; y--) {
      if (forrest[y][x] > highest) {
        visible.add(`${x}-${y}`);
        highest = forrest[y][x];
      }
    }
  }

  return visible.size;
}

const scenicScore = (x, y, forrest) => {
  const h = forrest.length;
  const w = forrest[0].length;
  const treeHeight = forrest[y][x];

  let left = 0;
  for (let i = x-1; i >= 0; i--) {
    left += 1;
    if (forrest[y][i] >= treeHeight) break
  }

  let right = 0;
  for (let i = x+1; i <= w-1; i++) {
    right += 1;
    if (forrest[y][i] >= treeHeight) break
  }

  let up = 0;
  for (let i = y-1; i >= 0; i--) {
    up += 1;
    if (forrest[i][x] >= treeHeight) break
  }

  let down = 0;
  for (let i = y+1; i <= h-1; i++) {
    down += 1;
    if (forrest[i][x] >= treeHeight) break
  }

  return left * right * up * down;
}

const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const forrest = data.map(line => line.split("").map(v => +v));
  const h = forrest.length;
  const w = forrest[0].length;
  let mostScenic = 0;

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      mostScenic = Math.max(mostScenic, scenicScore(x, y, forrest))
    }
  }

  return mostScenic;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()