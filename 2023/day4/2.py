import itertools

file = "input.txt"

sum = 0
wins = []

with open(file, "r") as f:
    for i, line in enumerate(f, 1):
        line = line.strip()
        winning = line.split(':')[1].split('|')[0].strip().split(' ')
        numbers = line.split(':')[1].split('|')[1].strip().split(' ')
        winning = [x.strip() for x in winning]
        winning = [x for x in winning if x != '']
        numbers = [x.strip() for x in numbers]
        numbers = [x for x in numbers if x != '']
        count = 0
        for num in numbers:
            if num in winning: count+=1
        wins.append(count)

print(f'wins: {wins}')
counts = [1 for x in range(len(wins))]
print(f'counts: {counts}')
for i in range(len(wins)):
    w = wins[i]
    c = counts[i]
    for j in range(i+1, min(i+w+1, len(wins))):
        counts[j] += c
    print(counts)

for count in counts:
    sum += count
print(f'sum: {sum}')
