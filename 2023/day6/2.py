input = "input.txt"

import numpy as np

with open(input, "r") as f:
    lines = f.readlines()
    time = int(''.join(lines[0].strip().split()[1:]))
    maxDistance = int(''.join(lines[1].strip().split()[1:]))

print(time)
print(maxDistance)

count = 0
for i in range(1, time-1):
    dist = i * (time-i)
    if (dist > maxDistance): count+=1
total = count
print(total)
