with open('2019/aoc-day06.txt', 'r') as f:
    data = f.readlines()


class Planet:
    def __init__(self, orbits, name=None):
        self.name = name
        self.parent = orbits

    def chksum(self):
        if isinstance(self.parent, str):
            return 1
        return self.parent.chksum() + 1

    def path(self):
        if isinstance(self.parent, str):
            return [self.name, self.parent]
        return [self.name] + self.parent.path()


def part_1(data):
    data = [x.strip().split(')') for x in data]
    planets = {b: Planet(a) for a, b in data}
    for planet in planets.values():
        planet.parent = planets.get(planet.parent, planet.parent)

    print(f'sum: {sum(x.chksum() for x in planets.values())}')


def part_2(data):
    data = [x.strip().split(')') for x in data]
    planets = {b: Planet(a, name=b) for a, b in data}
    for planet in planets.values():
        planet.parent = planets.get(planet.parent, planet.parent)

    a = set(planets['YOU'].path())
    b = set(planets['SAN'].path())
    print(f'Path: {len(a ^ b) - 2}')


if __name__ == '__main__':
    part_2(data)
