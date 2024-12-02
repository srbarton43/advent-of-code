input = "input.txt"
import numpy as np

prec = {
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "T": 10,
        "J": 11,
        "Q": 12,
        "K": 13,
        "A": 14,
        }

hands = []
count = 1
with open(input, "r") as f:
    for line in f:
        line = line.strip()
        hand, bid = line.split()
        hands.append((hand, int(bid)))
        #if count == 30: break
        count += 1

def getType(hand):
    counts = {}
    for c in hand:
        if c not in counts:
            counts[c] = 0
        counts[c]+=1
    m = 0
    vals = list(counts.values())
    vals.sort(reverse=True)
    match len(vals):
        case 1:
            return 7
        case 2:
            if vals[0] == 4:
                return 6
            else:
                return 5
        case 3:
            if vals[0] == 3:
                return 4
            else:
                return 3
        case 4:
            return 2
        case 5:
            return 1
    
    exit()
    return m

def aLess(a, b):
    for (x, y) in zip(a, b):
        if prec[x] > prec[y]:
            return False
        elif prec[x] < prec[y]:
            return True
    return False

sorted = []
for hand, bid in hands:
    t = getType(hand)
    print(f'hand: {hand}, type {t}')
    for i in range(len(sorted)):
        print(f's: {sorted[i][1]}, t: {t}')
        if sorted[i][1] < t: 
            sorted.insert(i, (hand, t, bid))
            break
        elif sorted[i][1] == t and aLess(sorted[i][0], hand):
            sorted.insert(i, (hand, t, bid))
            break
    else:
        sorted.append((hand, t, bid))
    print(np.array(sorted))

sorted = list(reversed(sorted))
print(np.array(sorted))
total = 0
for rank, (_, _, bid) in enumerate(sorted, 1):
    total += rank * bid

print(f'total: {total}')
