
def getPossibleResults(list, index):
    valid = list[index-25:index]
    solutions = []

    for e in valid:
        for n in valid:
            if e is not n and (x := e+n) not in solutions:
                solutions.append(x)

    return solutions
                
# part 1

with open('day9.txt') as f:

    index = 25
    solution = 0

    l = [int(x) for x in f.readlines()]

    print

    while True:
        if l[index] not in getPossibleResults(l, index):
            solution = l[index]
            break
        
        index += 1

    print(f"Broke at index {index} with number {solution}")


# part 2

with open('day9.txt') as f2:

    sum_index = 0
    start = 0
    stop = 0

    while True:

        tmp_index = sum_index
        tmp_sum = 0

        # checks for each starting index
        while tmp_sum <= solution:
            tmp_sum += l[tmp_index]

            if tmp_sum == solution:
                start = sum_index
                stop = tmp_index
                break

            tmp_index += 1

        if stop:
            break

        sum_index += 1

print(f"Start: {start} and Stop: {stop}")

s = l[start:stop]

print(f"Solution = {max(s) + min(s)}")