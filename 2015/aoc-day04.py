import hashlib


def part_1(data):
    hashed = ''
    val = 0
    while not hashed.startswith('00000'):
        val += 1
        hashed = str(hashlib.md5(f'{data}{val}'.encode('utf-8')).hexdigest())
    return val


def part_2(data):
    hashed = ''
    val = 0
    while not hashed.startswith('000000'):
        val += 1
        hashed = str(hashlib.md5(f'{data}{val}'.encode('utf-8')).hexdigest())
    return val


if __name__ == "__main__":
    print(f'part 2: {part_2("yzbqklnj")}')
