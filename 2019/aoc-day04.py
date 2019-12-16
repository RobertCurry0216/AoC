def part_1(start, limit):
    s1 = int(start[0])
    start = int(start)
    limit = int(limit)
    count = 0

    for d1 in range(s1, 10):
        for d2 in range(d1, 10):
            for d3 in range(d2, 10):
                for d4 in range(d3, 10):
                    for d5 in range(d4, 10):
                        for d6 in range(d5, 10):
                            num = f'{d1}{d2}{d3}{d4}{d5}{d6}'
                            if int(num) < start:
                                continue
                            if int(num) >= limit:
                                print(f'Count: {count}')
                                return count

                            for a in set(num):
                                if num.count(a) >= 2:
                                    count += 1
                                    break


def part_2(start, limit):
    s1 = int(start[0])
    start = int(start)
    limit = int(limit)
    count = 0

    for d1 in range(s1, 10):
        for d2 in range(d1, 10):
            for d3 in range(d2, 10):
                for d4 in range(d3, 10):
                    for d5 in range(d4, 10):
                        for d6 in range(d5, 10):
                            num = f'{d1}{d2}{d3}{d4}{d5}{d6}'
                            if int(num) < start:
                                continue
                            if int(num) >= limit:
                                print(f'Count: {count}')
                                return count

                            for a in set(num):
                                if num.count(a) == 2:
                                    count += 1
                                    break


part_2('265275', '781584')
