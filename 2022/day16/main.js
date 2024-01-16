const fs = require("fs");

const createMap = (filePath) => {
  const re = /Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)/;
  return fs.readFileSync(filePath, 'utf8')
  .split("\n")
  .reduce((acc, line)=>{
    const m = line.match(re);
    acc[m[1]] = {
      rate: +m[2],
      tunnels: m[3].split(", ")
    }
    return acc;
  }, {});
};

const mostPresureWrapper = () => {
  const cache = {};

  const mostPressure = (map, addr, time) => {
    if (time <= 0) return 0;
    if (typeof (cache[addr] || [])[time] == "number") return cache[addr][time];

    const location = map[addr];
    const curValve = location.rate * (time-1);
    let best = 0;
    for (const tunnel of location.tunnels) {
      best = Math.max(best, mostPressure(map, tunnel, time-1));
      best = Math.max(best, curValve + mostPressure(map, tunnel, time-2));
    }

    if (!cache[addr]) cache[addr] = [];
    cache[addr][time] = best;

    return best;
  };

  return mostPressure;
};

const part1 = (filePath) => {
  const map = createMap(filePath);
  const loctions = Object.keys(map);
  const mostPressure = mostPresureWrapper();
  const steps = 30;
  for (let i=1; i<steps; i++) {
    loctions.forEach(
      (l) => mostPressure(map, l, i)
    )
  }
  return mostPressure(map, "AA", steps);
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8');
  return 0;
}


const main = () => {
  console.log(`part 1: ${part1("./sample.txt")}`)
  console.log(`part 2: ${part2("./sample.txt")}`)
}

main()