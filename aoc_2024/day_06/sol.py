def next_pos(direction, pos, map):
    if direction % 4 == 0: # up
        next_pos = (pos[0] - 1, pos[1])
        if map[next_pos] == False:
            pos = next_pos
        else:
            direction += 1

    if direction % 4 == 1: # right
        next_pos = (pos[0], pos[1] + 1)
        if map[next_pos] == False:
            pos = next_pos
        else:
            direction += 1

    if direction % 4 == 2: # down
        next_pos = (pos[0] + 1, pos[1])
        if map[next_pos] == False:
            pos = next_pos
        else:
            direction += 1

    if direction % 4 == 3: # left
        next_pos = (pos[0], pos[1] - 1)
        if map[next_pos] == False:
            pos = next_pos
        else:
            direction += 1

    return direction, pos

def is_loop(direction, pos, map, loop_max_len=10_000):
    try:
        for _ in range(loop_max_len):
            direction, pos = next_pos(direction, pos, map)
        return True
    except KeyError:
        return False

map = dict()
start_pos = (0, 0)
max_j = 0
max_i = 0
# with open("day_06/toy_input.txt", "r") as f:
with open("day_06/input.txt", "r") as f:
    i = 0
    for line in f:
        stripped_line = line.strip()
        for j in range(len(stripped_line)):
            if stripped_line[j] == "#":
                map[(i, j)] = True # obstacle
            else:
                map[(i, j)] = False # no obstacle

            if stripped_line[j] == "^":
                start_pos = (i, j) # position

            max_j = max(max_j, j)
            max_i = max(max_i, i)

        i += 1


seen_positions = set()
pos = start_pos
direction = 0 # 0 = up, 1 = right, 2 = down, 3 = left
try:
    while True:
        seen_positions.add(pos)
        direction, pos = next_pos(direction, pos, map)
except:
    print("part 1:", len(seen_positions))


loop_cnt = 0
for i in range(max_i+1):
    for j in range(max_j+1):
        if map[(i, j)] == False and (i, j) != start_pos: # no obstacle
            map[(i, j)] = True
            if is_loop(0, start_pos, map):
                loop_cnt += 1
                print(f"loop found {loop_cnt}")
            map[(i, j)] = False # return map to original state

print("part 2:", loop_cnt)
