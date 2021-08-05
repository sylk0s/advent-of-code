treesHit = 0
relativePos = 0
over = 3 #spaces traveled right for each line down

for line in open("Day-3/day3.txt"):
    if line[relativePos] == "#":
        treesHit += 1
    relativePos += over
    if relativePos >= 31: #length of lines using len() didn't work?
        relativePos -= 31

print(treesHit)

over = [1,3,5,7] #spaces traveled right for each line down
amounts = []

for step in over:
    treesHit = 0
    relativePos = 0
    for line in open("Day-3/day3.txt"):
        if line[relativePos] == "#":
            treesHit += 1
        relativePos += step
        if relativePos >= 31: #length of lines using len() didn't work?
            relativePos -= 31
    amounts.append(treesHit)

treesHit = 0
relativePos = 0
index=0

for line in open("Day-3/day3.txt"):
    if index%2 == 0:
        if line[relativePos] == "#":
            treesHit += 1
        relativePos += 1
        if relativePos >= 31: #length of lines using len() didn't work?
            relativePos -= 31
    index += 1
amounts.append(treesHit)

multiplier = 1

for num in amounts:
    multiplier *= num

print(multiplier)