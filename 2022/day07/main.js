const fs = require("fs");

const commandRe = /\$ (.+)/;

const newFolder = (parent) => {
  return {
    files: {},
    folders: {},
    parent
  }
}

const buildTree = (data) => {
  // discard first line
  data.shift();
  // build file tree
  const root = newFolder();
  let current = root;
  for (const line of data) {
    const match = line.match(commandRe);
    if (match) {
      const [command, arg] = match[1].split(" ");
      if (command === "cd") {
        switch (arg) {
          case "/":
            current = root;
            break;
          case "..":
            current = current.parent;
            break;
          default:
            current = current.folders[arg];
            break;
        }
      }
    } else {
      // ls output
      const [dirOrSize, asset] = line.split(" ");
      if (dirOrSize === "dir") {
        current.folders[asset] = newFolder(current);
      } else {
        current.files[asset] = +dirOrSize;
      }
    }

  }
  return root;
}

const dirSize = (cache, dir, name) => {
  if (cache[name]) return cache[name];

  let size = 0;
  for (const fileSize of Object.values(dir.files)) {
    size += fileSize
  }

  for (const [fileName, subdir] of Object.entries(dir.folders)) {
    size += dirSize(cache, subdir, `${name}/${fileName}`);
  }
  cache[name] = size;
  return size;
}

const part1 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const root = buildTree(data);

  const dirSizeCache = {};
  dirSize(dirSizeCache, root, "~");

  return Object.values(dirSizeCache).filter(v => v <= 100000).reduce((a,c) => a+c, 0);
}


const part2 = (filePath) => {
  const data = fs.readFileSync(filePath, 'utf8').split(/\n/);
  const root = buildTree(data);

  const dirSizeCache = {};
  dirSize(dirSizeCache, root, "~");

  const target = 30000000 - (70000000 - dirSizeCache["~"])
  const dirs = Object.values(dirSizeCache).filter(v => v >= target);
  dirs.sort((a,b) => a-b);

  return dirs[0];
}


const main = () => {
  console.log(`part 1: ${part1("./input.txt")}`)
  console.log(`part 2: ${part2("./input.txt")}`)
}

main()