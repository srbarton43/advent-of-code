
sum = 0

def isStar(ch):
    return ch == '*'

def getStart(e, r, c):
    return {(max(0, r-1), max(0, c-1)), (min(len(eng)-1, r+1), max(0, c-1)), (r, max(0, c-1))}

def getNEnd(e, r, c):
    return {(max(0, r-1), c), (min(len(eng)-1, r+1), c)}

def getEnd(e, r, c):
    return {(max(0, r-1), c), (min(len(eng)-1, r+1), c), (r, min(len(e[r])-1, c))}

fp = open(
        "input.txt",
        "r"
        )

eng = []
map = {}

for line in fp:
    line = line.strip()
    print(line)
    eng.append(line)
fp.close()

for r, row in enumerate(eng):
    isNum = False
    start = -1
    ccs = []
    for c, ch in enumerate(row):
        if ch.isdigit() and not isNum:
            start = c
            isNum = True
            for coord in getStart(eng, r, c):
                if isStar(eng[coord[0]][coord[1]]):
                    if coord not in map:
                        map[coord] = []
                    if coord not in ccs: ccs.append(coord)
        if isNum:
            if ch.isdigit():
                for coord in getNEnd(eng, r, c):
                    if isStar(eng[coord[0]][coord[1]]):
                        if coord not in map:
                            map[coord] = []
                        if coord not in ccs: ccs.append(coord)
            else:
                for coord in getEnd(eng, r, c):
                    if isStar(eng[coord[0]][coord[1]]):
                        if coord not in map:
                            map[coord] = []
                        if coord not in ccs: ccs.append(coord)
                if len(ccs) > 0:
                    for cc in ccs:
                        map[cc].append(int(row[start:c]))
                isNum = False
                ccs = []
    if isNum:
        for coord in getEnd(eng, r, len(eng[r])-1):
            if isStar(eng[coord[0]][coord[1]]):
                if coord not in map:
                    map[coord] = []
                if coord not in ccs: ccs.append(coord)
                
        if len(ccs) > 0:
            for cc in ccs:
                map[cc].append(int(row[start:c]))

print(map)

for key in map:
    if len(map[key]) == 2:
        sum += map[key][0] * map[key][1]

print(f'sum: {sum}')
