from itertools import cycle
from collections import deque

with open('day9.txt') as f:
    players, marbles = (int(num) for num in f.read().split() if num.isdigit())

# marbles *= 100
pl = [0] * players
c = deque([0])

for p, m in zip(cycle(range(players)), range(1, marbles+1)):
    if m % 23 != 0:
        c.rotate(2)
        c.append(m)
    else:
        c.rotate(-7)
        pl[p] += m + c.pop()
        
print(max(pl))
