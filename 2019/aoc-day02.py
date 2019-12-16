def part_1(data_in, in_1=12, in_2=2):
    data = data_in.copy()
    data[1] = in_1
    data[2] = in_2
    p = 0
    while True:
        op = data[p]
        if op == 99:
            return data[0]
        a, b, c = data[p+1: p+4]
        if op == 1:
            data[c] = data[b] + data[a]
        else:
            data[c] = data[b] * data[a]
        p += 4


def part_2(data):
    for noun in range(100):
        for verb in range(100):
            if part_1(data, noun, verb) == 19690720:
                return f'{noun}{verb}'


if __name__ == "__main__":
    with open('2019/aoc-day02.txt', 'r') as f:
        data = [int(x) for x in f.read().split(',')]
    print(f'Part_1: {part_1(data)}')
    print(f'Part_2: {part_2(data)}')
