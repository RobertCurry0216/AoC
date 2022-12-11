const fs = require("fs");

const add = (old, val) => (val == "old" ? old + old : old + val);
const mul = (old, val) => (val == "old" ? old * old : old * val);

const makeMonkeys = (data) => {
  const monkeys = [];

  while (data.length > 0) {
    const id = +data.shift().match(/Monkey (\d+)/)[1];
    const items = data
      .shift()
      .split(": ")[1]
      .split(",")
      .map((v) => +v);
    const opMatch = data
      .shift()
      .match(/Operation: new = old (\+|\*) (\d+|old)/);
    const op = opMatch[1] == "+" ? add : mul;
    const opValue = opMatch[2] == "old" ? opMatch[2] : +opMatch[2];
    const test = +data.shift().split("by ")[1];
    const ifTrue = +data.shift().split("monkey ")[1];
    const ifFalse = +data.shift().split("monkey ")[1];
    data.shift();
    monkeys.push({
      id,
      items,
      op,
      opValue,
      test,
      ifTrue,
      ifFalse,
      inspections: 0,
    });
  }

  return monkeys;
};

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, "utf8").split(/\n/);
  const monkeys = makeMonkeys(data);

  for (let i = 0; i < 20; i++) {
    for (let j = 0; j < monkeys.length; j++) {
      const m = monkeys[j];
      while (m.items.length > 0) {
        m.inspections += 1;
        let item = m.items.shift();
        item = m.op(item, m.opValue);
        item = Math.floor(Number(item / 3));

        if (item % m.test == 0) {
          monkeys[m.ifTrue].items.push(item);
        } else {
          monkeys[m.ifFalse].items.push(item);
        }
      }
    }
  }

  const inspections = monkeys.map((m) => m.inspections);
  inspections.sort((a, b) => b - a);

  return inspections[0] * inspections[1];
};

const gcd = (a, b) => b == 0 ? a : gcd(b, a % b);
const lcm = (a, b) => (a * b) / gcd(Math.max(a,b), Math.min(a, b));

const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, "utf8").split(/\n/);
  const monkeys = makeMonkeys(data);

  const factor = monkeys.reduce((acc, cur)=> lcm(acc, cur.test), monkeys[0].test)

  for (let i = 0; i < 10000; i++) {
    for (let j = 0; j < monkeys.length; j++) {
      const m = monkeys[j];
      while (m.items.length > 0) {
        m.inspections += 1;
        let item = m.items.shift();
        item = m.op(item, m.opValue);
        item = item % factor;

        if (item % m.test == 0) {
          monkeys[m.ifTrue].items.push(item);
        } else {
          monkeys[m.ifFalse].items.push(item);
        }
      }
    }
  }

  const inspections = monkeys.map((m) => m.inspections);
  inspections.sort((a, b) => b - a);

  return inspections[0] * inspections[1];
};

const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`);
  console.log(`part 2: ${part2("./input.txt")}`);
};

main();
