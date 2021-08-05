array = []

for line in open("Day-1/day1.txt"):
    array.append(line)

print("-=-=-=- Puzzle 1 -=-=-=-")

for line1 in array:
    for line2 in array:
        if int(line1)+int(line2) == 2020:
            print(line1, " + ", line2)
            print(int(line1)*int(line2))

print("-=-=-=- Puzzle 2 -=-=-=-")

for line1 in array:
    for line2 in array:
        for line3 in array:
            if int(line1)+int(line2)+int(line3) == 2020:
                print(line1, " + ", line2, " + ", line3)
                print(int(line1)*int(line2)*int(line3))
