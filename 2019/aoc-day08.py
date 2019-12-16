def split_string(string, amt):
    while len(string) > 0:
        out, string = string[:amt], string[amt:]
        yield out


def part_1(data):
    min_0, max_val = 9999999, 0
    for sub in split_string(data, 25*6):
        if sub.count('0') < min_0:
            min_0 = sub.count('0')
            max_val = sub.count('1') * sub.count('2')
    return max_val


def part_2(data):
    layers = list(split_string(data, 25*6))
    layers.reverse()
    image = [2] * 25 * 6
    for layer in layers:
        for i in range(25 * 6):
            if layer[i] != '2':
                image[i] = layer[i]
    image_out = ''
    for i, val in enumerate(image):
        if not i % 25:
            image_out += '\n'
        image_out += '\u2593' if val == '1' else ' '
    print(image_out)


if __name__ == '__main__':
    with open('2019/aoc-day08.txt', 'r') as f:
        data = f.read().strip()
    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')
