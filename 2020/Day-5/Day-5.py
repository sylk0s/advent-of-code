results = []

for line in open("Day-5/day5.txt"):
    bi = 0b0
    for i in range(0,10):
        if line[i] == 'B' or line[i] == "R":
            place = 9-i
            bi += 0b1 << place
    MASK = 0b1111111000
    id = int((bi&MASK)>>3) * 8 + int(bi&~MASK)
    results.append(id)
print("Part 1: " + str(max(results)))

for i in range(min(results), max(results) + 1):
    if not i in results and results[i-1] and results[i+1]:
        print("Part 2: " + str(i))