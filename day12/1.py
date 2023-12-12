input = "sample.txt"
input = "input.txt"

from itertools import chain, combinations

def main():
    with open(input, "r") as f:
        sum = 0
        for line in f:
            line = line.strip()
            sum += doStuff(line)
    print(f'output: {sum}')

def doStuff(line):
    print(line)
    arrs = 0
    recs = [int(x) for x in line.split()[1].split(',')]
    raw = line.split()[0]
    print(raw)
    print(recs)
    qs = raw.count("?")
    tries = getBruteForce(raw, qs)
    for s in tries:
        if works(s, recs): arrs += 1
    print(arrs)
    return arrs

def getBruteForce(raw, qCount):
    pset = list(powerset(list(range(qCount))))
    tries = []
    for p in pset:
        pt = 0
        new = []
        for ch in raw:
            if ch == '?':
                new.append('.' if pt in p else '#') 
                pt+=1
            else:
                new.append(ch)
        tries.append(''.join(new))
    #print(tries)
    return tries

def powerset(iterable):
    s = list(iterable)
    return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))

def works(s, recs):
    i = 0
    print(s, recs)
    for rec in recs:
        val = rec
        while i < len(s):
            if s[i] == '#':
                val -= 1
            else:
                if val != rec:
                    if val != 0: return False
                    else: 
                        i += 1
                        break
            i += 1
        else:
            if val != 0:
                return False
    while i < len(s):
        if s[i] == '#': return False
        i+=1
    print("works!")
    return True
                    
                    

if __name__ == "__main__":
    main()
