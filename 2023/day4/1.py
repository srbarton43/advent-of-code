import itertools

file = "input.txt"

sum = 0

with open(file, "r") as f:
    for line in f:
        line = line.strip()
        winning = line.split(':')[1].split('|')[0].strip().split(' ')
        print(winning)
        numbers = line.split(':')[1].split('|')[1].strip().split(' ')
        print(numbers)
        winning = [x.strip() for x in winning]
        winning = [x for x in winning if x != '']
        numbers = [x.strip() for x in numbers]
        numbers = [x for x in numbers if x != '']
        count = 0
        for num in numbers:
            if num in winning: count+=1
        if count != 0:
            sum += (1 << (count-1))

print(f'sum: {sum}')
