def part_1(data):
    return data.count('(') - data.count(')')


def part_2(data):
    floor = 0
    for i, j in enumerate(data, 1):
        floor += 1 if j == '(' else -1
        if floor == -1:
            return i


if __name__ == "__main__":
    with open('2015/aoc-day01.txt', 'r') as f:
        data = f.read().strip()
    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')
