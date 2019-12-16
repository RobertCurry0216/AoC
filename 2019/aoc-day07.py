from itertools import permutations, cycle
from intcode import Intcode


def part_1(data):
    max_signal = 0
    for inputs in permutations((0, 1, 2, 3, 4)):
        prev = 0
        for i in inputs:
            prev = Intcode(data).eval(user_input=[prev, i])
        max_signal = max(max_signal, prev)
    print(f'Final signal: {max_signal}')


def part_2(data):
    max_signal = 0
    for inputs in permutations((5, 6, 7, 8, 9)):
        amps = [Intcode(data).eval_feedback(user_input=[i])
                for i in inputs]
        for amp in cycle(amps):
            try:
                next(amp)
            except StopIteration as e:
                max_signal = max(max_signal, Intcode.feedback)
                Intcode.feedback = 0
                break

    print(f'Max signal: {max_signal}')


if __name__ == '__main__':
    with open('2019/aoc-day07.txt', 'r') as f:
        data = f.read().strip()
    part_1(data)
    part_2(data)
