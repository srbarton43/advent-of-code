input = "sample.txt"
#input = "input.txt"

from itertools import chain, combinations

def main():
    with open(input, "r") as f:
        sum = 0
        for line in f:
            line = line.strip()
            sum += doStuff(line)
    print(f'output: {sum}')

def doStuff(line):
    #print(line)
    arrs = 0
    recs = [int(x) for x in line.split()[1].split(',')]
    raw = line.split()[0]
    
    print(raw)
    print(recs)

    return arrs
                    

if __name__ == "__main__":
    main()
