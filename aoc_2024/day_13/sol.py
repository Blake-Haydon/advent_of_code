import numpy as np
import re

A_TOKENS = 3
B_TOKENS = 1


def solve(x1, y1, x2, y2, x_target, y_target):
    A = np.array([[x1, x2], [y1, y2]])
    B = np.array([x_target, y_target])

    # solve the simultaneous equations
    sol = np.linalg.solve(A, B)

    [a_sol, b_sol] = sol
    a_sol = round(a_sol)
    b_sol = round(b_sol)

    return (a_sol, b_sol)

def is_solution(x1, y1, x2, y2, x_target, y_target, a_sol, b_sol):
    """check the solution is correct after rounding"""
    return (x1*a_sol + x2*b_sol) == int(x_target) and (y1*a_sol + y2*b_sol) == int(y_target)


# with open("day_13/toy_input.txt", "r") as f:
with open("day_13/input.txt", "r") as f:
    text = f.read()
    pattern = r"X[+=](\d+), Y[+=](\d+)"
    matches = re.findall(pattern, text)
    nums = [int(num) for pair in matches for num in pair]

total_tokens_p1 = 0
total_tokens_p2 = 0
for i in range(0, len(nums), 6):
    x1 = nums[i]
    y1 = nums[i+1]
    x2 = nums[i+2]
    y2 = nums[i+3]

    x_target = nums[i+4]
    y_target = nums[i+5]

    # part 1
    x_target_p1 = x_target
    y_target_p1 = y_target
    a_sol_p1, b_sol_p1 = solve(x1, y1, x2, y2, x_target_p1, y_target_p1)
    if is_solution(x1, y1, x2, y2, x_target_p1, y_target_p1, a_sol_p1, b_sol_p1):
        total_tokens_p1 += A_TOKENS*a_sol_p1 + B_TOKENS*b_sol_p1

    # part 2
    x_target_p2 = x_target + 10000000000000
    y_target_p2 = y_target + 10000000000000
    a_sol_p2, b_sol_p2 = solve(x1, y1, x2, y2, x_target_p2, y_target_p2)
    if is_solution(x1, y1, x2, y2, x_target_p2, y_target_p2, a_sol_p2, b_sol_p2):
        total_tokens_p2 += A_TOKENS*a_sol_p2 + B_TOKENS*b_sol_p2


print("part 1:", total_tokens_p1)
print("part 2:", total_tokens_p2)
