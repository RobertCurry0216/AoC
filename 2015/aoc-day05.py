def is_nice(string):
    if sum(x in 'aeiou' for x in string) < 3:
        return False
    if sum(i == j for i, j in zip(string, string[1:])) == 0:
        return False
    if sum(x in string for x in ('ab', 'cd', 'pq', 'xy')) > 0:
        return False
    return True


def part_1(data):
    return sum(map(is_nice, data))


if __name__ == "__main__":
    with open('2015/aoc-day05.txt', 'r') as f:
        data = f.readlines()
    print(f'part 1: {part_1(data)}')
    # print(f'part 2: {part_2(data)}')
