inputs = [int(line.strip()) for line in open("input.txt")]


def day_01_puzzle_1(vals):
    return sum(b > a for a, b in zip(vals[:-1], vals[1:]))


def day_01_puzzle_2(vals):
    # We only need to compare the edges (0 and 3), because 1 and 2 are shared
    window_size = 3
    return sum(x < y for x, y in zip(vals, vals[window_size:]))


print(f"Solution for Day 1 Puzzle 1: {day_01_puzzle_1(inputs)}")
print(f"Solution for Day 1 Puzzle 2: {day_01_puzzle_2(inputs)}")
