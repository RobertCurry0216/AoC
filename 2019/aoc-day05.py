class Intcode:
    def __init__(self, data):
        self.pointer = 0
        self.data = [int(x) for x in data.split(',')]

    def eval(self):
        op = self.data[0]

        op_codes = {
            1: self.op_1,
            2: self.op_2,
            3: self.op_3,
            4: self.op_4,
            5: self.op_5,
            6: self.op_6,
            7: self.op_7,
            8: self.op_8
        }

        while op != 99:
            c, b, a, *_ = f'{op:05}'
            op = op % 100
            op_codes[op](int(a), int(b), int(c))
            op = self.data[self.pointer]

        print(f'Halt: {self.data[0]}')
        return self.data[0]

    def _inputs(self, params):
        _, *out = self.data[self.pointer: self.pointer + params + 1]
        self.pointer += params + 1
        if len(out) > 1:
            return out
        return out[0]

    def op_1(self, *args):
        # plus
        mode_a, mode_b, mode_c = args
        a, b, c = self._inputs(3)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        self.data[c] = A + B

    def op_2(self, *args):
        # mul
        mode_a, mode_b, mode_c = args
        a, b, c = self._inputs(3)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        self.data[c] = A * B

    def op_3(self, *args):
        # input
        a = self._inputs(1)
        data_in = int(input('input: '))
        self.data[a] = data_in

    def op_4(self, *args):
        # output
        mode_a, *_ = args
        a = self._inputs(1)
        data_out = a if mode_a else self.data[a]
        print(f'output: {data_out}')

    def op_5(self, *args):
        # jump-in-true
        mode_a, mode_b, mode_c = args
        a, b = self._inputs(2)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        if A:
            self.pointer = B

    def op_6(self, *args):
        # jump-in-false
        mode_a, mode_b, mode_c = args
        a, b = self._inputs(2)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        if not A:
            self.pointer = B

    def op_7(self, *args):
        # less than
        mode_a, mode_b, mode_c = args
        a, b, c = self._inputs(3)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        if A < B:
            self.data[c] = 1
        else:
            self.data[c] = 0

    def op_8(self, *args):
        # equal
        mode_a, mode_b, mode_c = args
        a, b, c = self._inputs(3)
        A = a if mode_a else self.data[a]
        B = b if mode_b else self.data[b]
        if A == B:
            self.data[c] = 1
        else:
            self.data[c] = 0


if __name__ == '__main__':
    with open('2019/aoc-day05.txt', 'r') as f:
        i = Intcode(f.read())
    i.eval()
