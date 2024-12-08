def all_increasing(x):
    for i in range(len(x) - 1):
        if x[i] >= x[i + 1]:
            return False
    return True

def all_decreasing(x):
    for i in range(len(x) - 1):
        if x[i] <= x[i + 1]:
            return False
    return True

def max_adjacent(x):
    max_adj = 0
    for i in range(len(x) - 1):
        max_adj = max(max_adj, abs(x[i] - x[i + 1]))
    return max_adj


p1_score = 0
with open("day_02/input.txt", "r") as f:
    for line in f:
        x = [int(num) for num in line.split()]

        all_dec_or_inc = all_decreasing(x) or all_increasing(x)
        max_adj = max_adjacent(x)

        if all_dec_or_inc and max_adj <= 3:
            p1_score += 1

print("part 1:", p1_score)

p2_score = 0
with open("day_02/input.txt", "r") as f:
    for line in f:
        x = [int(num) for num in line.split()]

        for i in range(len(x)):
            x_rm_1 = x.copy()
            x_rm_1.pop(i)

            all_dec_or_inc = all_decreasing(x_rm_1) or all_increasing(x_rm_1)
            max_adj = max_adjacent(x_rm_1)

            if all_dec_or_inc and max_adj <= 3:
                p2_score += 1
                break

print("part 2:", p2_score)
