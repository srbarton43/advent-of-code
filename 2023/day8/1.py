input = "input.txt"

import pprint as pp
import re

map = {}

with open(input, "r") as f:
    lines = f.readlines()
    instr = lines[0].strip()
    for line in lines[2:]:
        line = line.strip()
        key = line.split('=')[0].strip()
        l, r = [x.strip() for x in line.split('=')[1].strip()[1:-1].split(',')]
        map[key] = (l, r)

print(instr)
pp.pprint(map)

cur = 'AAA'
exit()
p = 0
steps = 0
while True:
    print(cur)
    if cur == 'ZZZ':
        break
    if p == len(instr): p = 0
    move = instr[p]
    match move:
        case 'R':
            cur = map[cur][1]
        case 'L':
            cur = map[cur][0]
    steps += 1
    p += 1

print(f'steps: {steps}')
