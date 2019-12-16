class Star:
    stars = []
    area = float('inf')
    message = None
    time = 0
    
    def __init__(self, x, y, vx, vy):
        self.x = x
        self.y = y
        self.vx = vx
        self.vy = vy
        Star.stars.append(self)
        
    def move(self):
        self.x += self.vx
        self.y += self.vy
    
    @classmethod
    def incTime(cls):
        for s in cls.stars:
            s.move()

    @classmethod
    def getArea(cls):
        w1, w2, h1, h2 = cls.getExtents()
        return (w2 - w1) * (h2 - h1)

    @classmethod
    def getExtents(cls):
        w1 = min({s.x for s in cls.stars})
        w2 = max({s.x for s in cls.stars}) + 1
        h1 = min({s.y for s in cls.stars})
        h2 = max({s.y for s in cls.stars}) + 1
        return w1, w2, h1, h2
    
    @classmethod
    def render(cls, p):
        w1, w2, h1, h2 = cls.getExtents()
        s = ''
        for y in range(h1, h2):
            for x in range(w1, w2):
                s = s + '#' if (x, y) in p else s + ' '
            s = s + '\n'
        print(s)

    @classmethod
    def getPoints(cls):
        return {(s.x, s.y) for s in cls.stars}

    @classmethod
    def getMessage(cls):
        while cls.getArea() < cls.area:
            cls.area = cls.getArea()
            cls.message = cls.getPoints()
            cls.incTime()
            cls.time += 1
        cls.render(cls.message)

import re
with open('day10.txt') as f:
    pattern = re.compile(r'position=<(.+)> velocity=<(.+)>')
    for line in f.readlines():
        m = pattern.search(line)
        x, y = m.group(1).split(',')
        vx, vy = m.group(2).split(',')
        Star(int(x), int(y), int(vx), int(vy))
        
Star.getMessage()
