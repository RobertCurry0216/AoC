# get data
with open('day7.txt') as f:
    rules = []
    steps = set()
    for line in f.readlines():
        rules.append((ord(line[5]), ord(line[36])))
        steps.add(ord(line[5]))
        steps.add(ord(line[36]))

# order steps
def next_step(steps, rules, completed):
    aval = set(steps)
    for rule, step in rules:
        if rule not in completed:
            aval.discard(step)
    if len(aval) > 0:
        return min(aval)
    else:
        return None

completed = set()
s = ''

#part 1

##while len(steps) > 0:
##    n_step = next_step(steps, rules, completed)
##    completed.add(n_step)
##    steps.discard(n_step)
##    s = s + chr(n_step)

##print(s)

def second_smallest(numbers):
    m1, m2 = float('inf'), float('inf')
    for x in set(numbers):
        if x <= m1:
            m1, m2 = x, m1
        elif x < m2:
            m2 = x
    return m2

# part 2
class Worker:
    def __init__(self):
        self.time = 0
        self.job = 0

# create crew
workers = [Worker() for _ in range(5)]

while len(steps) > 0:
#for i in range(15):
    w = min(workers, key=lambda x: x.time) 
    completed.add(w.job)
    nj = next_step(steps, rules, completed)
    if nj:
        w.job = next_step(steps, rules, completed)
        w.time += (w.job - 4)
        steps.discard(w.job)
        s = s + chr(w.job)
    else:
        times = [t.time for t in workers]
        t = second_smallest(times)
        for a in workers:
            a.time = t if a.time < t else a.time
            if a.time == t:
                completed.add(a.job)
    
print(max([w.time for w in workers]))
    
