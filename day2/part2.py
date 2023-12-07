
file = open("input.txt", "r")

ret = 0


for line in file.readlines():
    line = line.strip()
    print(line)
    _, sets = line.split(':')
    
    # get sets
    sets = sets.split(';')
    # print(sets)

    minRed = 0
    minGreen = 0
    minBlue = 0
    
    for s in sets:
        # print(s)
        counts = s.strip().split(',')
        for count in counts:
            ct, color = count.strip().split(' ')
            ct = int(ct)
            match color:
                case 'red':
                    minRed = max(minRed, ct)
                case 'green':
                    minGreen = max(minGreen, ct)
                case 'blue':
                    minBlue = max(minBlue, ct)
    # if min(minRed, minGreen, minBlue) == 0: break

    ret += minRed * minGreen * minBlue

print(f'sum: {ret}')
