def is_value_for_pt1(comb_nums):
    base = 2
    n_ops = len(comb_nums) - 1
    for ith_comb in range(base**n_ops):
        temp_num = comb_nums[0]
        for i in range(n_ops):
            ith_comb_operation = ith_comb % base
            ith_comb //= base

            if ith_comb_operation == 0:
                temp_num += comb_nums[i+1]
            if ith_comb_operation == 1:
                temp_num *= comb_nums[i+1]

        if temp_num == test_num:
            return True

    return False


def is_value_for_pt2(comb_nums):
    base = 3
    n_ops = len(comb_nums) - 1
    for ith_comb in range(base**n_ops):
        temp_num = comb_nums[0]
        for i in range(n_ops):
            ith_comb_operation = ith_comb % base
            ith_comb //= base

            if ith_comb_operation == 0:
                temp_num += comb_nums[i+1]
            if ith_comb_operation == 1:
                temp_num *= comb_nums[i+1]
            if ith_comb_operation == 2:
                temp_num = int(str(temp_num) + str(comb_nums[i+1]))

        if temp_num == test_num:
            return True

    return False


correct_total_pt1 = 0
correct_total_pt2 = 0
# with open("day_07/toy_input.txt", "r") as f:
with open("day_07/input.txt", "r") as f:
    for line in f:
        split_line = line.strip().split(" ")
        test_num = int(split_line[0].strip(":"))
        comb_nums = [int(num) for num in split_line[1:]]
        if is_value_for_pt1(comb_nums):
            correct_total_pt1 += test_num
        if is_value_for_pt2(comb_nums):
            correct_total_pt2 += test_num

print("part 1:", correct_total_pt1)
print("part 2:", correct_total_pt2)
