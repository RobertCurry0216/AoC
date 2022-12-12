const fs = require("fs");

const createNode = (x, y, c) => {
  return {
    x, y,
    id: `${x}-${y}`,
    value: c == c.toLowerCase() ? c.charCodeAt() - 96 : (c == "S" ? 1 : 26),
    dist: Number.MAX_VALUE
  }
}

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(l => l.split(""));
  let s, e;

  // create graph
  const graph = {};
  for (let y = 0; y<data.length; y++) {
    for (let x=0; x<data[y].length; x++) {
      const v = data[y][x];
      const n = createNode(x, y, v)
      graph[n.id] = n;
      if (v == "S") {
        s = n;
      } else if (v == "E") {
        e = n;
      }
    }
  }

  // dijkstra
  s.dist = 0;
  const visited = new Set();
  const toVisit = [s];

  for (let i=0; i<data.length*data[0].length; i++) {
    toVisit.sort((a, b) => a.dist - b.dist);
    const c = toVisit.shift();
    visited.add(c.id);

    if (c.id == e.id) break

    // add links
    for (const [dx, dy] of [[-1, 0], [0, -1], [1, 0], [0, 1]]) {
      const n = graph[`${c.x + dx}-${c.y + dy}`];
      if (n && n.value <= c.value+1 && !visited.has(n.id)) {
        if (toVisit.every(v => v.id != n.id)) toVisit.push(n);
        n.dist = Math.min(n.dist, c.dist + 1);
      }
    }
  }

  return e.dist;
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(l => l.split(""));
  let s;

  // create graph
  const graph = {};
  for (let y = 0; y<data.length; y++) {
    for (let x=0; x<data[y].length; x++) {
      const v = data[y][x];
      const n = createNode(x, y, v)
      graph[n.id] = n;
      if (v == "E") {
        s = n;
      }
    }
  }

  // dijkstra
  s.dist = 0;
  const visited = new Set();
  const toVisit = [s];

  for (let i=0; i<data.length*data[0].length; i++) {
    if (toVisit.length == 0) break;

    toVisit.sort((a, b) => a.dist - b.dist);
    const c = toVisit.shift();
    visited.add(c.id);

    // add links
    for (const [dx, dy] of [[-1, 0], [0, -1], [1, 0], [0, 1]]) {
      const n = graph[`${c.x + dx}-${c.y + dy}`];
      if (n && n.value >= c.value-1 && !visited.has(n.id)) {
        if (toVisit.every(v => v.id != n.id)) toVisit.push(n);
        n.dist = Math.min(n.dist, c.dist + 1);
      }
    }
  }

  const steps = Object.values(graph).filter(n => n.value == 1).map(n => n.dist);
  steps.sort((a,b) => a-b);
  return steps[0];
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()