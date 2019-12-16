def checkMulti(text):
    twice = False
    thrice = False
    letters = set(text)
    for letter in letters:
        if text.count(letter) == 2:
            twice = True
        elif text.count(letter) == 3:
            thrice = True
    return (twice, thrice)

def compare_ids(id1, id2):
    count = 0
    for a, b in zip(id1, id2):
        if a != b:
            count += 1
            if count > 1:
                return False
    return True

def find_match(id_list):
    for i, id_val in enumerate(id_list):
        for id_comp in id_list[i + 1:]:
            if compare_ids(id_val, id_comp):
                return ''.join([i for i, j in zip(id_val, id_comp) if i == j])

id_list = []
with open(r'day2.txt', 'r') as data:
    twice = 0
    thrice = 0

    for line in data.readlines():
        _2, _3 = checkMulti(line)
        twice += _2
        thrice += _3
        if _2 or _3:
            id_list.append(line.rstrip())

    print(f'checksum: {twice * thrice}')
    print(f'common letters between 2 correct id\'s: {find_match(id_list)}')

