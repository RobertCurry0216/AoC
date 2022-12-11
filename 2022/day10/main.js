const fs = require("fs");

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, "utf8").split(/\n/);

  const keyCycles = [20, 60, 100, 140, 180, 220];

  let regX = 1;
  let cycle = 0;
  let delay = 0;
  let inst = null;
  let val = 0;
  let sum = 0;

  const delays = {
    ["addx"]: 1,
    ["noop"]: 0,
  };

  while (data.length > 0 || delay > 0) {
    if (delay <= 0) {
      [inst, val] = data.shift().trim().split(" ");
      val = Number(val);
      delay = delays[inst];
    } else {
      delay -= 1;
    }
    cycle += 1;

    if (keyCycles.includes(cycle)) {
      sum += regX * cycle;
    }

    if (delay <= 0) {
      switch (inst) {
        case "addx":
          regX += val;
          break;
      }
    }
  }

  return sum;
};

const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, "utf8").split(/\n/);

  let regX = 1;
  let cycle = 0;
  let delay = 0;
  let inst = null;
  let val = 0;
  const screen = new Array(240);

  const delays = {
    ["addx"]: 1,
    ["noop"]: 0,
  };

  while (data.length > 0 || delay > 0) {
    if (delay <= 0) {
      [inst, val] = data.shift().trim().split(" ");
      val = Number(val);
      delay = delays[inst];
    } else {
      delay -= 1;
    }
    cycle += 1;

    if (Math.abs((cycle % 40) - (regX + 1)) <= 1) {
      screen[cycle - 1] = "■";
    } else {
      screen[cycle - 1] = " ";
    }

    if (delay <= 0) {
      switch (inst) {
        case "addx":
          regX += val;
          break;
      }
    }
  }

  console.log(screen.slice(0, 40).join(""));
  console.log(screen.slice(40, 80).join(""));
  console.log(screen.slice(80, 120).join(""));
  console.log(screen.slice(120, 160).join(""));
  console.log(screen.slice(160, 200).join(""));
  console.log(screen.slice(200, 240).join(""));

  return "na";
};

const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`);
  console.log(`part 2: ${part2("./input.txt")}`);
};

main();
