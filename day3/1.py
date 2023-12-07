
sum = 0

def isSymbol(ch):
    return not ch.isdecimal() and ch != '.'

fp = open(
        "input.txt",
        "r"
        )

eng = []

for line in fp:
    line = line.strip()
    print(line)
    eng.append(line)
fp.close()

for r, row in enumerate(eng):
    isNum = False
    start = -1
    valid = False
    for c, ch in enumerate(row):
        if ch.isdecimal() and not isNum:
            start = c
            isNum = True
            if (isSymbol(eng[max(0, r-1)][max(0, c-1)])             # check beginning
                or isSymbol(eng[min(len(eng)-1, r+1)][max(0, c-1)]) 
                or isSymbol(eng[r][max(0, c-1)])):
                valid = True
        if isNum:
            if ch.isdecimal():
                if (isSymbol(eng[max(0, r-1)][c])                   # check beginning
                    or isSymbol(eng[min(len(eng)-1, r+1)][c]) 
                    or isSymbol(eng[r][c])):
                    valid = True
            else:
                isNum = False                                       # check end
                if (isSymbol(eng[max(0, r-1)][c])                   # check beginning
                    or isSymbol(eng[min(len(eng)-1, r+1)][c]) 
                    or isSymbol(eng[r][c])):
                    valid = True
                if valid:
                    sum += int(row[start:c])
                valid = False
    if isNum:
        if(isSymbol(eng[max(0, r-1)][len(eng[0])-1])
           or isSymbol(eng[min(len(eng)-1, r+1)][len(eng[0])-1])):
            valid = True
        if valid:
            sum += int(row[start:])

print(f'sum: {sum}')
