def is_valid(pos, grid):
    return 0 <= pos[0] < len(grid) and 0 <= pos[1] < len(grid[0])

def is_next(i, pos, grid):
    """return all the points that are accessible from the current
    position and the current index"""

    if not is_valid(pos, grid):
        return set()

    if int(grid[pos[0]][pos[1]]) == i:
        if int(i) == 9:
            return set([pos])

        else:
            up_set = is_next(i+1, (pos[0]-1, pos[1]), grid)
            down_set = is_next(i+1, (pos[0]+1, pos[1]), grid)
            left_set = is_next(i+1, (pos[0], pos[1]-1), grid)
            right_set = is_next(i+1, (pos[0], pos[1]+1), grid)

            return up_set.union(down_set).union(left_set).union(right_set)

    return set()

def is_next_cnt(i, pos, grid):
    """return the total paths that are accessible from the current
    position and the current index"""

    if not is_valid(pos, grid):
        return 0

    if int(grid[pos[0]][pos[1]]) == i:
        if int(i) == 9:
            return 1

        else:
            up = is_next_cnt(i+1, (pos[0]-1, pos[1]), grid)
            down = is_next_cnt(i+1, (pos[0]+1, pos[1]), grid)
            left = is_next_cnt(i+1, (pos[0], pos[1]-1), grid)
            right = is_next_cnt(i+1, (pos[0], pos[1]+1), grid)

            return up + down + left + right

    return 0


grid = []
with open("day_10/input.txt", "r") as f:
    for line in f:
        grid.append(line.strip())

# find all 0 starting points
start_points = []
for i in range(len(grid)):
    for j in range(len(grid[i])):
        if grid[i][j] == "0":
            start_points.append((i, j))

# use `is_next` to find all the points that are accessible
total_p1 = 0
for point in start_points:
    total_p1 += len(is_next(0, point, grid))
print("part 1:", total_p1)


# use `is_next_cnt` to find the total number of paths
total_p2 = 0
for point in start_points:
    total_p2 += is_next_cnt(0, point, grid)
print("part 2:", total_p2)
