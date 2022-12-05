const fs = require("fs");

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8');
  const values = data.split(/\n/);

  let max = 0;
  let currentElf = 0;

  values.forEach(value => {
    if (value === "") {
      max = Math.max(max, currentElf);
      currentElf = 0;
    } else {
      currentElf += +value;
    }
  });
  return max;
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8');
  const values = data.split(/\n/);

  const sums = [];
  let currentElf = 0;

  values.forEach(value => {
    if (value === "") {
      sums.push(currentElf);
      currentElf = 0;
    } else {
      currentElf += +value;
    }
  });
  sums.sort((a,b) => b-a);
  return sums[0] + sums[1] + sums[2];
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()