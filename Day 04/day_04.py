import numpy as np

inputs = [(line.strip()) for line in open("input.txt") if len(line.strip()) > 0]
draw = np.array(inputs[0].split(","))
draw = draw.astype(np.int32)
tables = np.array(
    [[table.split() for table in inputs[i : i + 5]] for i in range(1, len(inputs), 5)]
)
tables = tables.astype(np.int32)


def day_04_puzzle_1(draw, tables):
    marks = np.zeros_like(tables)

    def calc_unmarked(table_ix):
        it = np.nditer(marks[table_ix], flags=["multi_index"])
        return np.sum(
            [(tables[(table_ix, *it.multi_index)]) for mark in it if mark == 0]
        )

    for called_num in draw:
        # Iterate over every element in tables
        it = np.nditer(tables, flags=["multi_index"])
        for tables_num in it:
            # Place mark
            if called_num == tables_num:
                marks[(it.multi_index)] = 1

                # Check win condition
                table_ix, x, y = it.multi_index
                if np.all(marks[table_ix, x, :]) or np.all(marks[table_ix, :, y]):
                    return called_num * calc_unmarked(table_ix)


def day_04_puzzle_2(vals):
    marks = np.zeros_like(tables)
    win_order = []
    last_num = None

    def calc_unmarked(table_ix):
        it = np.nditer(marks[table_ix], flags=["multi_index"])
        return np.sum(
            [(tables[(table_ix, *it.multi_index)]) for mark in it if mark == 0]
        )

    for called_num in draw:
        # Iterate over every element in tables
        it = np.nditer(tables, flags=["multi_index"])
        for tables_num in it:
            table_ix, x, y = it.multi_index
            if table_ix in win_order:
                continue

            # Place mark
            if called_num == tables_num:
                marks[(it.multi_index)] = 1

                # Check win condition
                if np.all(marks[table_ix, x, :]) or np.all(marks[table_ix, :, y]):
                    win_order.append(table_ix)
                    last_num = called_num

    return calc_unmarked(win_order[-1]) * last_num


print(f"Solution for Day 4 Puzzle 1: {day_04_puzzle_1(draw, tables)}")
print(f"Solution for Day 4 Puzzle 2: {day_04_puzzle_2(inputs)}")
