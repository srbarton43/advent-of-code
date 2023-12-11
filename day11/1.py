input = "sample.txt"
#input = "input.txt"

from pprint import pprint

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

tGrid = grid.copy()
r = 0
while r < len(tGrid):
    row = tGrid[r]
    if rowIsEmpty(row):
        tGrid.insert(r, ['.' for c in row])
        r+=1
    r+=1

c = 0
while c < len(tGrid[0]):
    if colIsEmpty(tGrid, c):
        for row in tGrid:
            row.insert(c, '.')
        c+=1
    c+=1

pprint(tGrid)

# transformed grid
grid = tGrid
gals = []

for r, row in enumerate(tGrid):
    for c, ch in enumerate(row):
        if ch == '#':
            gals.append((r, c))

sum = 0
print(gals)
for i in range(len(gals)):
    for j in range(i+1, len(gals)):
        sum += abs(gals[i][0]-gals[j][0]) + abs(gals[i][1] - gals[j][1])

print(sum)
