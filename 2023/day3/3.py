import itertools
import numpy as np

def getVal(arr, r, c):
    start = end = c
    while start >= 0 and arr[r][start].isdigit(): start-=1
    while end < len(arr[r]) and arr[r][end].isdigit(): end+=1
    x = int("".join(arr[r][start+1:end]))
    for c in range(start+1, end):
        arr[r][c] = '_'
    return x

def pAdj(arr, r, c):
    arr = [row[:] for row in arr]
    ratios = []
    x = list(range(r-1, r+2))
    y = list(range(c-1, c+2))
    for x, y in itertools.product(x, y):
        if x >= 0 and x < len(arr) and y >= 0 and y < len(arr[r]):
            if (arr[x][y].isdigit()):
                ratio = getVal(arr, x, y)
                ratios.append(ratio)
    
    if len(ratios) == 2:
        return ratios[0]*ratios[1]
    else:
        return 0
                
    print(arr)
arr = []
sum = 0
with open("input.txt", "r") as f:
    for line in f:
        line = line.strip()
        line = list(line)
        print(line)
        arr.append(line)

for r, row in enumerate(arr):
    for c, col in enumerate(row):
        if col == '*':
            sum += pAdj(arr, r, c)

print(f'sum: {sum}')
