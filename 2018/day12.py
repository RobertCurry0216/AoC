import re
buffer = 10000
with open('day12.txt') as f:
    __, pots_s = f.readline().rsplit(maxsplit=1)
    pots_s = '.'*10 + pots_s + '.'*buffer
    f.readline()
    data = f.read().split('\n')
    tests = set()
    for test in data:
        rule, result = test.split(' => ')
        rule = f'(?<={re.escape(rule[:2])})({re.escape(rule[2])})(?={re.escape(rule[3:])})'
        tests.add((rule, result))

# apply rules
last_total = 0
for j in range(0, 1000, 1):
    for __ in range(1):
        pots_a = ['.'] * (len(pots_s))
        for rule, result in tests:
            matches = re.finditer(rule, pots_s)
            for m in matches:
                pots_a[m.start()] = result
        pots_s = "".join(pots_a)

    # sum total
    total = 0
    for i, p in enumerate(pots_s):
        if p == '#':
            total += (i - 10)
    print(f'iteration: {j + 1}  -  count: {total}  -  diff: {total - last_total}')
    last_total = total
