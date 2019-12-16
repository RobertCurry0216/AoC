## part 1

##with open('day8.txt') as f:
##    data = f.read()
##
##def next_int(data):
##    while len(data) > 1:
##        n, data = data.split(maxsplit=1)
##        yield int(n)
##    yield int(data)
##
##def branch(d):
## 
##    child = next(d)
##    meta = next(d)
##    total = 0
##    for __ in range(child):
##        total += branch(d)
##
##    for __ in range(meta):
##        total += next(d)
##
##    return total
##
## main
##d = next_int(data)
##print(branch(d))

## part 2

with open('day8.txt') as f:
    data = f.read()

def next_int(data):
    while len(data) > 1:
        n, data = data.split(maxsplit=1)
        yield int(n)
    yield int(data)

def branch(d):
    child = next(d)
    meta = next(d)
    total = 0
    if child:
        child_values = [branch(d) for __ in range(child)]
        for __ in range(meta):
            i = next(d)
            try:
                total += child_values[i-1]
            except:
                pass
    else:
        for __ in range(meta):
            total += next(d)
    return total

# main
print(branch(next_int(data)))
