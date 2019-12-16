with open('day5.txt', 'r') as f:
    data = f.read().strip()

from string import ascii_lowercase as LC
from string import ascii_uppercase as UC

def collapse(data):
    l1 = len(data)
    l2 = l1 + 1

    while l1 != l2:
        l2 = l1
        for c in LC:
            data = data.replace(c+c.upper(), '').replace(c.upper()+c, '')
        l1 = len(data)
    return len(data)
        

def full_collapse(data):
    min_len = 20000
    min_chr = 'a'
    for c in LC:
        data_improved = data.replace(c, '').replace(c.upper(), '')
        length = collapse(data_improved)
        if length < min_len:
            min_len = length
            min_chr = c
    return (min_len, min_chr)

print(full_collapse(data))
