lines = []
for line in open("Day-6/day6.txt"):
    lines.append(line)
lines.append("\n")

count = 0
localAnswered = []
localCount = 0
for line in lines:
    if line == "\n":
        count += localCount
        localAnswered = []
        localCount = 0
    else:
        for char in line:
            if not char in localAnswered and not char == "\n":
                localAnswered.append(char)
                localCount += 1

print("Part 1: " + str(count))
group = []
count2 = 0

for line in lines:
    if line == "\n":
        #print(set.intersection(*map(set, group)))
        count2 += len(set.intersection(*map(set, group))) #map applied the set function to all in group, * means that it is passing through a dynamic sized list
        group = []

    else:
        group.append(line[:-1]) #this is because of \n in every line
print("Part 2: " + str(count2))
                