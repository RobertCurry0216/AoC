const fs = require("fs");

const doesOverlapEntirely = (d) => (d.x <= d.u && d.y >= d.v) || (d.u <= d.x && d.v >= d.y)

const doesOverlapAtAll = (d) => (d.x <= d.v && d.y >= d.u) || (d.u <= d.y && d.v >= d.x)

const part1 = (filePath) => {
  const re = /(\d+)-(\d+),(\d+)-(\d+)/
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(sections => {
    const match = sections.match(re);
    return {x: +match[1], y: +match[2], u: +match[3], v: +match[4]}
  });
  return data.filter(doesOverlapEntirely).length;
}


const part2 = (filePath) => {
  const re = /(\d+)-(\d+),(\d+)-(\d+)/
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(sections => {
    const match = sections.match(re);
    return {x: +match[1], y: +match[2], u: +match[3], v: +match[4]}
  });
  return data.filter(doesOverlapAtAll).length;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()