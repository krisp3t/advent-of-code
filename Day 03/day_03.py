from collections import Counter

inputs = [(line.strip()) for line in open("input.txt")]


def day_03_puzzle_1(vals):
    num_length = len(vals[0])
    gamma = epsilon = 0
    for ix in range(num_length):
        count = Counter(row[ix] for row in vals)
        weight = 2 ** (num_length - ix - 1)
        gamma += int(count.most_common()[0][0]) * weight  # Gamma bit (most common)
        epsilon += (
            int(count.most_common()[-1][0]) * weight
        )  # Epsilon bit (least common)

    return gamma * epsilon


def day_03_puzzle_2(vals):
    def determine_rating(most_common=True):
        num_length = len(vals[0])
        legit_vals = vals
        for ix in range(num_length):
            count = Counter(row[ix] for row in legit_vals)
            most_common_bit, most_common_qty = count.most_common()[0]
            least_common_bit, least_common_qty = count.most_common()[-1]
            if most_common_qty == least_common_qty:
                most_common_bit = "1"
                least_common_bit = "0"
            if len(legit_vals) == 1:
                break
            legit_vals = [
                val
                for val in legit_vals
                if val[ix] == (most_common_bit if most_common else least_common_bit)
            ]
        return legit_vals[0]

    oxygen = determine_rating(True)
    co2 = determine_rating(False)
    return int(oxygen, 2) * int(co2, 2)


print(f"Solution for Day 3 Puzzle 1: {day_03_puzzle_1(inputs)}")
print(f"Solution for Day 3 Puzzle 2: {day_03_puzzle_2(inputs)}")
