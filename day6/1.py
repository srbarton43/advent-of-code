input = "input.txt"

import numpy as np

with open(input, "r") as f:
    lines = f.readlines()
    times = [int(x) for x in lines[0].strip().split()[1:]]
    distances = [int(x) for x in lines[1].strip().split()[1:]]

print(times)
print(distances)

ways = []
for t, maxDistance in zip(times, distances):
    count = 0
    for i in range(1, t-1):
        dist = i * (t-i)
        if (dist > maxDistance): count+=1
    ways.append(count)
print(ways)
total = np.product(ways)
print(total)
