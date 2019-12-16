def part_1(data):
    def area(x):
        a = x[0]*x[1]
        b = x[1]*x[2]
        c = x[2]*x[0]
        return sum((a, a, b, b, c, c, min(a, b, c)))
    return sum(map(area, data))


def part_2(data):
    def length(x):
        c = x.copy()
        c.remove(max(x))
        a, b = c
        return sum((2*a, 2*b, x[0]*x[1]*x[2]))
    return sum(map(length, data))


if __name__ == "__main__":
    with open('2015/aoc-day02.txt', 'r') as f:
        data = []
        for line in f.readlines():
            x, y, z = line.split('x')
            data.append([int(x), int(y), int(z)])

    #print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')
