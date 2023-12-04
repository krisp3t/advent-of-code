inputs = [(line.strip()) for line in open("input.txt")]


def day_02_puzzle_1(vals):
    x = y = 0
    for val in vals:
        direction, units = val.split()
        match (direction, int(units)):
            case ("forward", units):
                x += units
            case ("down", units):
                y += units
            case ("up", units):
                y -= units
    return x * y


def day_02_puzzle_2(vals):
    x = y = aim = 0
    for val in vals:
        direction, units = val.split()
        match (direction, int(units)):
            case ("forward", units):
                x += units
                y += units * aim
            case ("down", units):
                aim += units
            case ("up", units):
                aim -= units
    return x * y


print(f"Solution for Day 2 Puzzle 1: {day_02_puzzle_1(inputs)}")
print(f"Solution for Day 2 Puzzle 2: {day_02_puzzle_2(inputs)}")
