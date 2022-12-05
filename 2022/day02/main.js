const fs = require("fs");

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  let score = 0;
  data.forEach(game => {
    const elf = game.charCodeAt(0) - 64;
    const you = game.charCodeAt(2) - 87;
    let modifier = you - elf + 1;
    if (you + elf === 4) {
      if (you === 3) modifier = 0;
      if (you === 1) modifier = 2;
    }
    const result = modifier * 3;
    score += you + result;
  })
  return score;
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const target = [
    [3, 1, 2], // lose
    [1, 2, 3], // draw
    [2, 3, 1], // win
  ]
  let score = 0;
  data.forEach(game => {
    const elf = game.charCodeAt(0) - 64;
    const goal = game.charCodeAt(2) - 88;
    const you = target[elf - 1][goal];
    let modifier = you - elf + 1;
    if (you + elf === 4) {
      if (you === 3) modifier = 0;
      if (you === 1) modifier = 2;
    }
    const result = modifier * 3;
    score += you + result;
  })
  return score;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()