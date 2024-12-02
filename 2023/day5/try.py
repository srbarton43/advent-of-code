
with open("input.txt", "r") as f:
    
    mins = []
    mi = 90000000000
    for line in f:
        s = line.split()
        if s: 
            s = s[0]
            print(s)
            if s.isdigit() and int(s) > 0: 
                mi = min(mi, int(s))
                mins.append(mi)
    mins.sort()
    print(mins[:10])
