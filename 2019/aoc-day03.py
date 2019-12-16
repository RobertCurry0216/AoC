file = '2019/aoc-day03.txt'
# file = '2019/sample.txt'

with open(file, 'r') as f:
    wire_1, wire_2 = [x.strip() for x in f.readlines()]


class Vec(tuple):

    def next(self, heading, dist):
        if heading == 'U':
            point = Vec((self[0], self[1] + dist))
            return point, Wire(self, point)
        elif heading == 'D':
            point = Vec((self[0], self[1] - dist))
            return point, Wire(self, point)
        elif heading == 'R':
            point = Vec((self[0] + dist, self[1]))
            return point, Wire(self, point)
        else:
            point = Vec((self[0] - dist, self[1]))
            return point, Wire(self, point)

    def distance(self, other):
        return abs(self[0] - other[0]) + abs(self[1] - other[1])


class Wire:
    def __init__(self, sp, ep):
        self.sp = sp
        self.ep = ep

    def __repr__(self):
        return f'Wire({self.sp}, {self.ep})'

    @property
    def is_vert(self):
        return self.sp[0] == self.ep[0]

    @property
    def length(self):
        return abs(self.sp[0] - self.ep[0]) + abs(self.sp[1] - self.ep[1])

    def intersect(self, other):
        if self.is_vert:
            # vertical
            if self.sp[1] < other.sp[1] < self.ep[1] or self.sp[1] > other.sp[1] > self.ep[1]:
                if other.sp[0] < self.sp[0] < other.ep[0] or other.sp[0] > self.sp[0] > other.ep[0]:
                    return Vec((self.sp[0], other.sp[1]))
        else:
            # horizontal
            if self.sp[0] < other.sp[0] < self.ep[0] or self.sp[0] > other.sp[0] > self.ep[0]:
                if other.sp[1] < self.sp[1] < other.ep[1] or other.sp[1] > self.sp[1] > other.ep[1]:
                    return Vec((other.sp[0], self.sp[1]))

    def point_on(self, point):
        if self.sp[0] <= point[0] < self.ep[0] and self.sp[1] == point[1]:
            return abs(point[0] - self.sp[0])
        if self.sp[1] <= point[1] < self.ep[1] and self.sp[0] == point[0]:
            return abs(point[1] - self.sp[1])

        if self.sp[0] >= point[0] > self.ep[0] and self.sp[1] == point[1]:
            return abs(point[0] - self.sp[0])
        if self.sp[1] >= point[1] > self.ep[1] and self.sp[0] == point[0]:
            return abs(point[1] - self.sp[1])

        return False


def part_1(wire_1_data, wire_2_data):
    min_dist, min_point = 99999, None
    wire_1 = []
    sp = Vec((0, 0))
    for wire in wire_1_data.split(','):
        p, w = sp.next(wire[0], int(wire[1:]))
        wire_1.append(w)
        sp = p

    wire_2 = []
    sp = Vec((0, 0))
    for wire in wire_2_data.split(','):
        p, w = sp.next(wire[0], int(wire[1:]))
        wire_2.append(w)
        sp = p

    for vert in [x for x in wire_1 if x.is_vert]:
        for horiz in [y for y in wire_2 if not y.is_vert]:
            i = vert.intersect(horiz)
            if not i:
                continue
            d = i.distance(Vec((0, 0)))
            if d < min_dist:
                min_dist, min_point = d, i

    return min_dist, min_point


def part_2(wire_1_data, wire_2_data):

    def dist_to_point(wires, point):
        dist = 0
        for wire in wires:
            i = wire.point_on(point)
            if i:
                return dist + i
            dist += wire.length
        else:
            print(f'error: {wire}, {point}')

    wire_1 = []
    sp = Vec((0, 0))
    for wire in wire_1_data.split(','):
        p, w = sp.next(wire[0], int(wire[1:]))
        wire_1.append(w)
        sp = p

    wire_2 = []
    sp = Vec((0, 0))
    for wire in wire_2_data.split(','):
        p, w = sp.next(wire[0], int(wire[1:]))
        wire_2.append(w)
        sp = p

    min_dist = 99999
    for vert in [x for x in wire_1 if x.is_vert]:
        for horiz in [y for y in wire_2 if not y.is_vert]:
            i = vert.intersect(horiz)
            if not i:
                continue
            d = dist_to_point(wire_1, i) + dist_to_point(wire_2, i)
            if d < min_dist:
                min_dist = d
    return min_dist


if __name__ == "__main__":
    print(f'Distance to nearest intersection: {part_2(wire_1, wire_2)}')
