input = "input.txt"

with open(input, "r") as f:
    lines = [x.strip() for x in f.readlines()]

def getDiff(nums):
    ret = []
    for i in range(1, len(nums)):
        ret.append((nums[i]-nums[i-1]))
    return ret

def allZeroes(nums):
    for num in nums:
        if num != 0: return False
    return True

def getNext(nums):
    #print(nums)
    bt = [nums]
    while not allZeroes(nums):
        nums = getDiff(nums)
        bt.append(nums)
        #print(nums)
    #print(bt)
    prev = 0
    for ns in reversed(bt):
        #print(ns)
        prev += ns[-1]
    print(prev)
    return prev

sum = 0
for line in lines:
    print(line)
    sum += getNext([int(x) for x in line.split()])

print(f'total: {sum}')
