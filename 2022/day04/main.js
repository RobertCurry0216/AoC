const fs = require("fs");

const doesOverlapEntirely = (d) => (d.x <= d.u && d.y >= d.v) || (d.u <= d.x && d.v >= d.y)

const doesOverlapAtAll = (d) => (d.x <= d.v && d.y >= d.u) || (d.u <= d.y && d.v >= d.x)

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(sections => {
    const [a, b] = sections.split(",");
    const [x, y] = a.split("-")
    const [u, v] = b.split("-")
    return {x: Number(x), y: Number(y), u: Number(u), v: Number(v)}
  });
  return data.filter(doesOverlapEntirely).length;
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/).map(sections => {
    const [a, b] = sections.split(",");
    const [x, y] = a.split("-")
    const [u, v] = b.split("-")
    return {x: Number(x), y: Number(y), u: Number(u), v: Number(v)}
  });
  return data.filter(doesOverlapAtAll).length;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()