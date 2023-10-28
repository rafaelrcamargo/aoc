input = open("./assets/index.txt", "r").readlines()

crates = []

for line in input:
    if not line.startswith(" 1 "):
        crates.append(line.strip("\n"))
    else:
        break

stacks = [[] for _ in range(0, len(input[0]), 4)]

for i, line in enumerate(crates):
    row = [line[i : i + 4] for i in range(0, len(line), 4)]

    for ii, crate in enumerate(row):
        char = crate.strip("[] ")

        if char:
            stacks[ii].append(char)

moves = []

for line in input:
    if line.startswith("move"):
        moves.append([int(x) for i, x in enumerate(line.split()) if (i + 1) % 2 == 0])

for move in moves:
    list = stacks[move[1] - 1][0 : move[0]]
    list.reverse()  # For part 2.

    for i in list:
        stacks[move[2] - 1].insert(0, i)

    del stacks[move[1] - 1][0 : move[0]]


print("\nResult: ")
for stack in stacks:
    print(stack[0], end="")

print("\n")
