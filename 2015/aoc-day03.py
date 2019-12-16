def part_1(data):
    dirs = {'^': (0, 1), 'v': (0, -1), '<': (-1, 0), '>': (1, 0)}
    x, y = 0, 0
    houses = [(0, 0)]
    for i in data:
        dx, dy = dirs[i]
        x += dx
        y += dy
        houses.append((x, y))
    return len(set(houses))


def part_2(data):
    dirs = {'^': (0, 1), 'v': (0, -1), '<': (-1, 0), '>': (1, 0)}
    x = [0, 0]
    y = [0, 0]
    houses = [(0, 0)]
    for i, j in enumerate(data):
        dx, dy = dirs[j]
        x[i % 2] += dx
        y[i % 2] += dy
        houses.append((x[i % 2], y[i % 2]))
    return len(set(houses))


if __name__ == "__main__":
    with open('2015/aoc-day03.txt', 'r') as f:
        data = f.read().strip()
    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')
