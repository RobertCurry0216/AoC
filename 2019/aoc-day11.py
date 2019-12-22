from intcode import Intcode

DIRS = {0:(0,1), 1:(1,0), 2:(-1,0), 3:(-1,0)}

class HullPainter:
    def __init__(self, data):
        self.intcode = Intcode(data)
        self.x = 0
        self.y = 0
        self.dir = 0
        self.painted = set()
    
    @property
    def location(self):
        return (self.x, self.y)

    def move(self):
        _dir = DIRS[self.dir]
        self.x += _dir[0]
        self.y += _dir[1]
    
    def turn(self, right):
        if right:
            self.dir += 1
        else:
            self.dir -= 1
        self.dir = (self.dir + 4) % 4

    def paint(self, colour):
        if colour:
            self.painted.add(self.location)

    def paint_hull(self):
        computer = self.intcode.eval_feedback(feedback_mode=False)
        while True:
            try:
                #set current location input
                self.intcode.user_input = [int(self.location in self.painted)]
                # paint current location
                p = next(computer)
                self.paint(p)
                # turn
                d = next(computer)
                self.turn(d)
                # move 1 square
                self.move()
            except StopIteration:
                return


def part_1(data):
    robot = HullPainter(data)
    return f'painted squares: {robot.paint_hull()}'

def part_2(data):
    pass

if __name__ == '__main__':
    with open('2019/aoc-day11.txt', 'r') as f:
        data = f.read()
    print(f'part 1: {part_1(data)}')
    print(f'part_2: {part_2(data)}')
