def part_1(data):
    def calc(num):
        return num // 3 - 2
    return sum(map(calc, data))


def part_2(data):
    def calc(num):
        out = num // 3 - 2
        return 0 if out <= 0 else out + calc(out)
    return sum(map(calc, data))


if __name__ == "__main__":
    with open('2019/aoc-day01.txt', 'r') as f:
        data = [int(x) for x in f.readlines()]
    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')
