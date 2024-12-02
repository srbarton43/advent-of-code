input = "sample.txt"
input = "input.txt"

from pprint import pprint

EXP = 1000000

grid = []
with open(input, "r") as f:
    for line in f:
        line = line.strip()
        grid.append([x for x in line])

pprint(grid)

def rowIsEmpty(row):
    for ch in row:
        if ch == '#': return False
    return True

def colIsEmpty(grid, c):
    for row in grid:
        if row[c] == '#': return False
    return True

gals = []

rFactor = 0
actualRows = []

for r, row in enumerate(grid):
    if rowIsEmpty(row):
        rFactor += EXP-1
    actualRows.append(r+rFactor)

cFactor = 0
actualCols = []
for c in range(len(grid[0])):
    if colIsEmpty(grid, c):
        cFactor += EXP-1
    actualCols.append(cFactor+c)

for r, row in enumerate(grid):
    for c, ch in enumerate(row):
        if ch == '#':
            gals.append((actualRows[r], actualCols[c]))
print(actualRows, actualCols)

sum = 0
print(gals)
for i in range(len(gals)):
    for j in range(i+1, len(gals)):
        sum += abs(gals[i][0]-gals[j][0]) + abs(gals[i][1] - gals[j][1])

print(sum)
