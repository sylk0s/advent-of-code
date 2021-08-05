n = 0

for line in open("Day-2/day2.txt"):
    data = line.split(" ")
    range = data[0].split("-")
    letter = data[1][0]
    passwd = data[2]

    occurances = 0

    for char in passwd:
        if char == letter:
            occurances += 1

    if occurances >= int(range[0]) and occurances <= int(range[1]):
        n += 1

print(n)

i = 0

for line in open("Day-2/day2.txt"):
    data = line.split(" ")
    indexs = data[0].split("-")
    letter = data[1][0]
    passwd = data[2]

    hasFoundOnce = False
    hasFoundTwice = False

    for index in indexs:
        if passwd[int(index)-1] == letter:
            if not hasFoundTwice and hasFoundOnce:
                hasFoundTwice = True
            if not hasFoundOnce:
                hasFoundOnce = True

    if hasFoundOnce and not hasFoundTwice:
        i += 1

print(i)