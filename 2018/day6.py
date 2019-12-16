import csv
path = r'day6.txt'
with open(path, newline='') as file:
    reader = csv.reader(file)
    data = []
    for a, b in reader:
        data.append((int(a), int(b)))

size = 400

def find_closest(points, y, x):
    min_dist = 99999
    next_min = 99999
    nearest = None
    for px, py in points:
        dist = abs(x - px) + abs(y - py)
        if dist == min_dist:
            next_min = dist
        elif dist < min_dist:
            nearest = (px, py)
            min_dist = dist
    if min_dist == next_min:
        nearest = None
    return nearest

# part 1
##count_dict = {p:0 for p in data}
##for y in range(size):
##    for x in range(size):
##        try:
##            c = find_closest(data, y, x)
##            count_dict[c] += 1
##            # detect edges
##            if x==0 or y==0 or x==(size-1) or y==(size-1):
##                count_dict[c] = None
##        except:
##            pass

##print(max([i for i in count_dict.values() if i]))

# part 2

def find_ave_dist(points, y, x):
    dist = 0
    for px, py in points:
        dist += abs(x - px) + abs(y - py)
    return dist

area = 0
for y in range(size):
    for x in range(size):
        if find_ave_dist(data, y, x) < 10000:
            area += 1

print(area)
