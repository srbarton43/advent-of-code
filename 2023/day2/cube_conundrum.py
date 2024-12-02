
file = open("input.txt", "r")

MR = 12
MG = 13
MB = 14

ret = 0


for line in file.readlines():
    line = line.strip()
    print(line)
    game, sets = line.split(':')
    
    # get ID
    _, id = game.split(' ')
    id = int(id)
    # print(id)
    
    # get sets
    sets = sets.split(';')
    # print(sets)

    cur = id
    
    for s in sets:
        # print(s)
        counts = s.strip().split(',')
        for count in counts:
            ct, color = count.strip().split(' ')
            ct = int(ct)
            match color:
                case 'red':
                    if ct > MR: cur = 0
                case 'blue':
                    if ct > MB: cur = 0
                case 'green':
                    if ct > MG: cur = 0

    ret += cur

print(f'sum: {ret}')
