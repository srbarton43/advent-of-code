file = "sample.txt"
#file = "input.txt"

def main():
    with open(file, 'r') as fp:
        lines = [l.strip() for l in fp.readlines()]
    seeds = [int(x) for x in lines[0].split()[1:]]
    print(seeds)
    maps = readMaps(lines[2:])
    print(maps)
    calcMin(seeds, maps)

def readMaps(lines):
    maps = []
    for line in lines:
        print(line)
        if "map" in line:
            maps.append(dict())
        elif line != '':
            print(line)
            dst, src, l = [int(x) for x in line.split()]
            maps[-1][(dst, dst+l)] = src
    return maps

def calcMin(seeds, maps):
    loc = 0#800000000
         #863761171
    seedRanges = []
    for i in range(1, len(seeds), 2):
        seedRanges.append((seeds[i-1], seeds[i-1] + seeds[i]))
    #print(seedRanges)
    #m = seedRanges[0][0]
    #for mi, mx in seedRanges:
    #    m = min(m, mi)
    #print(m)
    while True:
        print(loc)
        seed = lookupRev(loc, maps)
        #print(seed)
        if goodSeed(seed, seedRanges): break
        loc += 1
    print(f'min loc: {loc}')

def lookupRev(location, maps):
    cur = location
    #print(f'location: {location}')
    for map in reversed(maps):
        for destRange in map:
            mi, mx = destRange  # [mi,mx)
            src = map[destRange]
            if cur >= mi and cur < mx:
                cur = src + (cur - mi)
                break
        else:
            cur = cur
        # print(f'intermediate: {cur}')
    return cur

def goodSeed(seed, seedRanges):
    for r in seedRanges:
        mi, mx = r
        if seed >= mi and seed < mx: return True
    return False

if __name__ == "__main__":
    main()

#potential min = 806029445

