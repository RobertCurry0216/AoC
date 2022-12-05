const fs = require("fs");

const extractData = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);

  // split stacks and actions
  const tempStacks = [];
  for (let i = 0; i < 10; i++) {
    const line = data.shift();
    if (line.match(/\d/)) {
      data.shift();// clear empty line
      break;
    } else {
      tempStacks.unshift(line);
    }
  }

  // build stacks 
  const stacks = new Array(10).fill(null).map(_ => []);
  for (let i = 0; i < tempStacks.length; i++) {
    const line = tempStacks[i];
    let stack = 0;
    for (let j = 1; j < line.length; j += 4) {
      const v = line[j];
      if (v != " ") stacks[stack].push(v)
      stack += 1;
    }
  }

  return [stacks, data];
}

const part1 = (filePath) => {
  const [stacks, actions] = extractData(filePath);
  const re = /move (\d+) from (\d+) to (\d+)/

  for (action of actions) {
    const match = action.match(re);
    const times = +match[1];
    const from = +match[2];
    const to = +match[3];

    for (let i = 0; i < times; i++) {
      const v = stacks[from-1].pop();
      stacks[to-1].push(v);
    }
  }

  return stacks.filter(s => s.length > 0).reduce((acc, cur)=> acc + cur[cur.length-1], "");
}


const part2 = (filePath) => {
  const [stacks, actions] = extractData(filePath);
  const re = /move (\d+) from (\d+) to (\d+)/

  for (action of actions) {
    const match = action.match(re);
    const times = +match[1];
    const from = +match[2];
    const to = +match[3];

    const crates = stacks[from-1].slice(-times);
    stacks[from-1] = stacks[from-1].slice(0, -times)
    stacks[to-1] = stacks[to-1].concat(crates);
  }

  return stacks.filter(s => s.length > 0).reduce((acc, cur)=> acc + cur[cur.length-1], "");
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()