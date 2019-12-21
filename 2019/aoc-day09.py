from intcode import Intcode


def part_1(data):
    print('Part 1 start:')
    return Intcode(data).eval(user_input=[1])

def part_2(data):
    print('Part 2 start:')
    return Intcode(data).eval(user_input=[2])


if __name__ == "__main__":
    with open('2019/aoc-day09.txt', 'r') as f:
        data = f.read().strip()
    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')