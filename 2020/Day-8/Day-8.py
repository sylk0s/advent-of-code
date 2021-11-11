with open('day8.txt') as instructions_file:
    inst = instructions_file.readlines()

    for i in inst:
        f2 = open('day8.txt')
        instructions = f2.readlines()
        if i[0:2] == "nop":
            instructions[instructions.index(i)] = "jmp " + i[3:]
        if i[0:2] == "jmp":
            instructions[instructions.index(i)] = "nop " + i[3:]

        pointer = 0
        executed = []
        accumulator = 0

        exec_instruction = 0

        instruction = instructions[pointer].split()

        while not pointer in executed and pointer <= len(instructions):
            executed.append(pointer)
            exec_instruction = pointer
            if instruction[0] == "acc":
                if instruction[1][0] == "+":
                    accumulator += int(instruction[1][1:])
                if instruction[1][0] == "-":
                    accumulator -= int(instruction[1][1:])

            if instruction[0] == "jmp":
                if instruction[1][0] == "+":
                    pointer += int(instruction[1][1:])
                elif instruction[1][0] == "-":
                    pointer -= int(instruction[1][1:])
            else:
                pointer += 1

            instruction = instructions[pointer].split()
        
        if pointer > len(instructions):
            print(instructions.index(i))
            print(pointer)
            print(accumulator)
