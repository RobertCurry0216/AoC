def get_power(x, y, serial):
    """"  Get the power level for a given fuel cell """
    if x == 0 or y == 0:
        return 0
    _id = x + 10
    power = _id * y + serial
    power *= _id
    power = (power // 100) % 10
    return power - 5


def max_nxn(grid, n):
    """ return the max value for a nxn section of a given grid """
    _max = 0
    _max_point = None
    for y in range(1, 301 - n):
        for x in range(1, 301 - n):
            A = grid[y][x]
            B = grid[y][x+n]
            C = grid[y+n][x]
            D = grid[y+n][x+n]
            total = A + D - B - C
            if total > _max:
                _max = total
                _max_point = (x+1, y+1)
    return _max_point, _max


def partial_sum_table():
    """ generate a partial sum table """
    grid_serial = 6303
    sum_grid = [[0 for _ in range(301)] for __ in range(301)]
    for y in range(1, 301):
        for x in range(1, 301):
            sum_grid[y][x] = get_power(x, y, grid_serial) \
                            + sum_grid[y-1][x] \
                            + sum_grid[y][x - 1] \
                            - sum_grid[y - 1][x - 1]
    return sum_grid


if __name__ == '__main__':
    grid = partial_sum_table()
    max_v = 0
    max_p = 0
    max_n = 0
    for n in range(3, 301):
        p, v = max_nxn(grid, n)
        if v > max_v:
            max_v = v
            max_p = p
            max_n = n
        print(f'loop {n} of 300')
    print(max_p, max_n)
