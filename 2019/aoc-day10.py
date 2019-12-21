from math import atan2, degrees, pi
from collections import defaultdict
from itertools import cycle

def count_visable(x1, y1, data):
    count = set()
    for y2, row in enumerate(data):
        for x2, asteroid in enumerate(row):
            if asteroid and (x1 != x2 or y1 != y2):
                count.add(int(degrees(atan2(y2-y1, x2-x1)) * 100 ))
    return len(count)
     
def part_1(data):
    _max, _x, _y = 0, 0, 0
    for y, row in enumerate(data):
        for x, asteroid in enumerate(row):
            if asteroid:
                c = count_visable(x, y, data)
                if c > _max:
                    _max, _x, _y = c, x, y
    return f'x:{_x}, y:{_y}, max:{_max}'

def part_2(data):
    x = 20
    y = 21

    # list all asteroids
    asteroids = defaultdict(list)
    for y2, row in enumerate(data):
        for x2, asteroid in enumerate(row):
            if asteroid and (x != x2 or y != y2):
                dy, dx = y2-y, x2-x
                deg = (540 + degrees(atan2(dy, dx) - pi/2)) % 360
                asteroids[(int(deg * 100))].append((x2, y2))
    # sort each list by distance
    for k, v in asteroids.items():
        asteroids[k] = sorted(v, key=lambda i: abs(i[0] - x) + abs(i[1] - y))
    # destroy first 200 asteroids
    count = 0
    for deg in cycle(sorted(asteroids.keys())):
        if asteroids[deg]:
            count += 1
            out = asteroids[deg].pop(0)
            #print(f'count:{count}, asteroid:{out}, deg:{deg/100}')

        if count >= 200:
            return out
        


if __name__ == '__main__':
    with open('2019/aoc-day10.txt', 'r') as f:
        data = [[i == '#' for i in j.strip()] for j in f.readlines()]

    print(f'part 1: {part_1(data)}')
    print(f'part 2: {part_2(data)}')