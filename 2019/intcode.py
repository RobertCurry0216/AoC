class InfList(list):
    def __getitem__(self, key):
        try:
            return list.__getitem__(self, key)
        except IndexError:
            while self.__len__() <= key:
                self.append(0)
            return list.__getitem__(self, key)

    def __setitem__(self, key, value):
        try:
            list.__setitem__(self, key, value)
        except IndexError:
            while self.__len__() <= key:
                self.append(0)
            list.__setitem__(self, key, value)


class Intcode:
    feedback = 0

    def __init__(self, data):
        self.pointer = 0
        self.rel_base = 0
        self.data = InfList([int(x) for x in data.split(',')])
        self.user_input = None
        self.data_out = None

    def _inputs(self, params):
        _, *out = self.data[self.pointer: self.pointer + params + 1]
        self.pointer += params + 1
        if len(out) > 1:
            return out
        return out[0]

    def _get_value(self, val, mode):
        # get the value based on the mode
        if mode == 0:
            return self.data[val]
        if mode == 1:
            return val
        if mode == 2:
            return self.data[self.rel_base + val]

    def _set_value(self, data_val, mode, val):
        #sets a memory value based on an mode
        if mode == 0:
            self.data[data_val] = val
        if mode == 1:
            raise ValueError('Mode 1 cannot be used to set')
        if mode == 2:
            self.data[self.rel_base + data_val] = val

    def eval(self, user_input=None):
        compute = self.eval_feedback(
            user_input=user_input, feedback_mode=False)
        while True:
            try:
                next(compute)
            except StopIteration:
                return self.data_out

    def eval_feedback(self, user_input=None, feedback_mode=True):
        op = self.data[0]
        self.user_input = user_input

        while op != 99:
            c, b, a, *_ = f'{op:05}'
            op = op % 100
            mode_a, mode_b, mode_c = int(a), int(b), int(c)

            if op == 1:
                # plus
                a, b, c = self._inputs(3)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                self._set_value(c, mode_c, A + B)

            elif op == 2:
                # mul
                a, b, c = self._inputs(3)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                self._set_value(c, mode_c, A * B)

            elif op == 3:
                # input
                a = self._inputs(1)
                if self.user_input:
                    data_in = self.user_input.pop()
                elif feedback_mode:
                    data_in = Intcode.feedback
                else:
                    data_in = int(input('input: '))

                self._set_value(a, mode_a, data_in)

            elif op == 4:
                # output
                a = self._inputs(1)
                data_out = self._get_value(a, mode_a)
                self.data_out = data_out
                if feedback_mode:
                    Intcode.feedback = self.data_out
                    yield
                print(f'Output: {data_out}')

            elif op == 5:
                # jump-in-true
                a, b = self._inputs(2)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                if A:
                    self.pointer = B

            elif op == 6:
                # jump-in-false
                a, b = self._inputs(2)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                if not A:
                    self.pointer = B

            elif op == 7:
                # less than
                a, b, c = self._inputs(3)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                if A < B:
                    self._set_value(c, mode_c, 1)
                else:
                    self._set_value(c, mode_c, 0)

            elif op == 8:
                # equal
                a, b, c = self._inputs(3)
                A = self._get_value(a, mode_a)
                B = self._get_value(b, mode_b)
                if A == B:
                    self._set_value(c, mode_c, 1)
                else:
                    self._set_value(c, mode_c, 0)

            elif op == 9:
                # adjust rel base
                a = self._inputs(1)
                A = self._get_value(a, mode_a)
                self.rel_base += A

            op = self.data[self.pointer]

        return self.data_out
