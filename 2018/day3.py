class Fabric():
    def __init__(self):
        from re import compile
        self.fabric = [[0 for __ in range(1000) ] for __ in range(1000)]
        self.pattern = compile('#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

    def render(self):
        for row in self.fabric:
            print(''.join([str(i) for i in row]))

    def cut(self, data):
        from re import search
        claim_id, x, y, w, h = search(self.pattern, data).groups()
        x = int(x)
        y = int(y)
        h = int(h)
        w = int(w)
        for i in range(x, x + w):
            for j in range(y, y + h):
                self.fabric[j][i] += 1

    def count_overlap(self):
        count = 0
        for row in self.fabric:
            for col in row:
                count += col > 1
        return count

    def check_overlap(self, data):
        from re import search
        claim_id, x, y, w, h = search(self.pattern, data).groups()
        x = int(x)
        y = int(y)
        h = int(h)
        w = int(w)
        for i in range(x, x + w):
            for j in range(y, y + h):
                if self.fabric[j][i] > 1:
                    return False
        return claim_id

f = Fabric()

with open('day3.txt', 'r') as f_data:
    for data in f_data.readlines():
        f.cut(data.rstrip())

with open('day3.txt', 'r') as f_data:
    for data in f_data.readlines():
        overlap = f.check_overlap(data.rstrip())
        if overlap:
            print(overlap)



