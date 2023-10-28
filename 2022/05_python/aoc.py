with open("./assets/input.txt", "r", encoding="utf8") as file:
    in_lines = file.readlines()

    def get_crates(lines: list[str]) -> list[str]:
        crates = []

        for line in lines:
            if not line.startswith(" 1 "):
                crates.append(line.strip("\n"))
            else:
                break

        return crates

    def get_stacks(lines: list[str]) -> list[list[str]]:
        stacks: list[list[str]] = [[] for _ in range(0, len(lines[0]), 4)]

        for i, line in enumerate(get_crates(lines)):
            row = [line[i : i + 4] for i in range(0, len(line), 4)]

            for ii, crate in enumerate(row):
                char = crate.strip("[] ")

                if char:
                    stacks[ii].append(char)

        return stacks

    def get_moves(lines: list[str]) -> list[list[int]]:
        moves = []

        for line in lines:
            if line.startswith("move"):
                moves.append(
                    [int(x) for i, x in enumerate(line.split()) if (i + 1) % 2 == 0]
                )

        return moves

    def solve_moves(lines: list[str], reverse: bool = False) -> list[list[str]]:
        stacks = get_stacks(lines)

        for move in get_moves(lines):
            crates = stacks[move[1] - 1][0 : move[0]]

            if reverse:
                crates.reverse()  # For part 2.

            for i in crates:
                stacks[move[2] - 1].insert(0, i)

            del stacks[move[1] - 1][0 : move[0]]

        return stacks

    print("\nPart 1: ")
    for stack in solve_moves(in_lines):
        print(stack[0], end="")

    print("\n\nPart 2: ")
    for stack in solve_moves(in_lines, reverse=True):
        print(stack[0], end="")

    print("\n")
