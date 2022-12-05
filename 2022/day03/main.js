const fs = require("fs");

const intersection = (set1, set2) => Array.from(set1).filter(x => set2.has(x))

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  let value = 0;
  data.forEach(rucksack => {
    const c1 = new Set(rucksack.split("").splice(0, rucksack.length/2));
    const c2 = new Set(rucksack.split("").splice(rucksack.length/2, rucksack.length/2));
    for (const el of intersection(c1, c2)) {
      let item;
      if (el.charCodeAt() < 96) {
        item = el.charCodeAt() - 38;
      } else {
        item = el.charCodeAt() - 96;
      }
      value += item
    }
  })
  return value;
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  let value = 0;
  for (let i = 0; i < data.length; i += 3) {
    const elf1 = new Set(data[i].split(""));
    const elf2 = new Set(data[i+1].split(""));
    const elf3 = new Set(data[i+2].split(""));
    const i12 = intersection(elf1, elf2);
    const i123 = intersection(i12, elf3);
    for (const el of i123) {
      let item;
      if (el.charCodeAt() < 96) {
        item = el.charCodeAt() - 38;
      } else {
        item = el.charCodeAt() - 96;
      }
      value += item
    }
  }
  return value;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()