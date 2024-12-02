input = "sample.txt"

def main():
    patterns = []
    pattern = []
    with open(input, "r") as f:
        for line in f:
            line = line.strip()
            if line == '':
                patterns.append(pattern)
                patern = []
            else:
                pattern.append(list(line))
        patterns.append(pattern)

    total = 0
    for pattern in patterns:
        total += findReflection(pattern)
    print(f'total: {total}')


def findReflection(pattern):
    print(pattern)
    score = tryVertical(pattern)
    if score == 0: score = tryHorizontal(pattern)
    return score

def tryVertical(pattern):
    left, right = 0, pattern
    return 0

def tryHorizontal(pattern):
    return 0

if __name__ == "__main__":
    main()
