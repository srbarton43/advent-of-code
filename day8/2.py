input = "input.txt"

import pprint as pp
import re
import time

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

starting = [ x for x in list(map.keys()) if re.match("[1-9A-Z]{2}A", x)]
print(starting)
p = 0
steps = 0
next = starting
yes = True
while yes:
    print(steps)
    prev = next
    next = []
    yes = False
    for i, cur in enumerate(prev):
        print(cur)
        if not re.match("[1-9A-Z]{2}Z", cur):
            yes = True
        if p == len(instr): p = 0
        move = instr[p]
        match move:
            case 'R':
                next.append(map[cur][1])
            case 'L':
                next.append(map[cur][0])
    steps += 1
    p += 1
    time.sleep(1)

print(f'steps: {steps}')
