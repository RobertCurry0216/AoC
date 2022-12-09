const fs = require("fs");

const moveKnotTowards = (head, tail) => {
  const dx = head.x - tail.x
  const dy = head.y - tail.y
  if (Math.abs(dx) > 1 || Math.abs(dy) > 1) {
    tail.x += Math.sign(dx);
    tail.y += Math.sign(dy);
  }
}

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const head = {x: 0, y:0}
  const tail = {x: 0, y:0}
  const tailSteps = new Set();

  for (const line of data) {
    const [dir, steps] = line.split(" ");
    for (let i=0; i< +steps; i++){
      // move head
      switch (dir) {
        case "U": head.y-=1; break;
        case "D": head.y+=1; break;
        case "L": head.x-=1; break;
        case "R": head.x+=1; break;
      }

      // update tail
      moveKnotTowards(head, tail)
      tailSteps.add(`(${tail.x},${tail.y})`)
    }
  }

  return tailSteps.size;
}

const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const knots = new Array(10).fill(null).map(_ => {return {x: 0, y:0}});
  const head = knots[0];
  const tail = knots[9];
  const tailSteps = new Set();

  for (const line of data) {
    const [dir, steps] = line.split(" ");
    for (let i=0; i< +steps; i++){
      // move head
      switch (dir) {
        case "U": head.y-=1; break;
        case "D": head.y+=1; break;
        case "L": head.x-=1; break;
        case "R": head.x+=1; break;
      }

      // update tail
      for (let i=1; i<knots.length; i++) {
        moveKnotTowards(knots[i-1], knots[i]);
      }
      tailSteps.add(`(${tail.x},${tail.y})`)
    }
  }

  return tailSteps.size;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()