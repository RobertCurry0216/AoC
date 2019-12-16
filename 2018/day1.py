data = open('day1.txt', 'r')
num = 0
freq = 0

#part 1
##for i in data.read().split('\n'):
##    num += int(i)
##
##print(num)

data_list = [int(i) for i in data.read().split('\n')]
past_data = set()
i = 0

while True:
    num = data_list[i]
    freq += num
    if freq in past_data:
        print(f'first value reached twice: {freq}')
        break
    else:
        past_data.add(freq)
        i = (i + 1) % len(data_list)
        if len(past_data) > 100000000:
            print('escape')
            break
