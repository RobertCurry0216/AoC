const fs = require("fs");

const part1 = (filePath) => {
  const re = /(.)(?!\1)(.)(?!\1|\2)(.)(?!\1|\2|\3)(.)/;
  const data = fs.readFileSync(filePath, 'utf8');
  return data.match(re)?.index + 4;
}


const part2 = (filePath) => {
  const re = /(.)(?!\1)(.)(?!\1|\2)(.)(?!\1|\2|\3)(.)(?!\1|\2|\3|\4)(.)(?!\1|\2|\3|\4|\5)(.)(?!\1|\2|\3|\4|\5|\6)(.)(?!\1|\2|\3|\4|\5|\6|\7)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\10)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\10|\11)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\10|\11|\12)(.)(?!\1|\2|\3|\4|\5|\6|\7|\8|\9|\10|\11|\12|\13)(.)/;
  const data = fs.readFileSync(filePath, 'utf8');
  return data.match(re)?.index + 14;
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()