input = open("./assets/index.txt", "r").readlines()


def get_crates(input):
    crates = []

    for line in input:
        if not line.startswith(" 1 "):
            crates.append(line.strip("\n"))
        else:
            break

    return crates


def get_stacks(input):
    stacks = [[] for _ in range(0, len(input[0]), 4)]

    for i, line in enumerate(get_crates(input)):
        row = [line[i : i + 4] for i in range(0, len(line), 4)]

        for ii, crate in enumerate(row):
            char = crate.strip("[] ")

            if char:
                stacks[ii].append(char)

    return stacks


def get_moves(input):
    moves = []

    for line in input:
        if line.startswith("move"):
            moves.append(
                [int(x) for i, x in enumerate(line.split()) if (i + 1) % 2 == 0]
            )

    return moves


def solve_moves(input, reverse=False):
    stacks = get_stacks(input)

    for move in get_moves(input):
        list = stacks[move[1] - 1][0 : move[0]]

        if reverse:
            list.reverse()  # For part 2.

        for i in list:
            stacks[move[2] - 1].insert(0, i)

        del stacks[move[1] - 1][0 : move[0]]

    return stacks


print("\nPart 1: ")
for stack in solve_moves(input):
    print(stack[0], end="")

print("\n\nPart 2: ")
for stack in solve_moves(input, reverse=True):
    print(stack[0], end="")

print("\n")
