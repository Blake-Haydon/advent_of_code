from itertools import combinations

def delta(loc_1, loc_2):
    return loc_1[0] - loc_2[0], loc_1[1] - loc_2[1]

def in_bounds(loc, max_i, max_j):
    return 0 <= loc[0] < max_i and 0 <= loc[1] < max_j

loc_dict = dict()
max_i = 0
max_j = 0
# with open("day_08/toy_input.txt", "r") as f:
with open("day_08/input.txt", "r") as f:
    j = 0
    for line in f:
        i = 0
        for c in line.strip():
            if c != ".":
                if c not in loc_dict:
                    loc_dict[c] = {(i, j)}
                else:
                    loc_dict[c].add((i, j))
            i += 1
            max_i = max(max_i, i)

        j += 1
        max_j = max(max_j, j)


antinote_locs_1 = set()
for key in loc_dict:
    for loc_1, loc_2 in combinations(loc_dict[key], 2):
        loc_delta = delta(loc_1, loc_2)

        # vector math
        loc_delta_1 = (loc_2[0] + 2*loc_delta[0], loc_2[1] + 2*loc_delta[1])
        loc_delta_2 = (loc_2[0] - 1*loc_delta[0], loc_2[1] - 1*loc_delta[1])
        if in_bounds(loc_delta_1, max_i, max_j):
            antinote_locs_1.add(loc_delta_1)
        if in_bounds(loc_delta_2, max_i, max_j):
            antinote_locs_1.add(loc_delta_2)

print("part 1:", len(antinote_locs_1))


antinote_locs_2 = set()
for key in loc_dict:
    for loc_1, loc_2 in combinations(loc_dict[key], 2):
        loc_delta = delta(loc_1, loc_2)

        # vector math
        for k in range(max_i):
            loc_delta_1 = (loc_2[0] + (k+1)*loc_delta[0], loc_2[1] + (k+1)*loc_delta[1])
            loc_delta_2 = (loc_2[0] - k*loc_delta[0], loc_2[1] - k*loc_delta[1])
            if in_bounds(loc_delta_1, max_i, max_j):
                antinote_locs_2.add(loc_delta_1)
            if in_bounds(loc_delta_2, max_i, max_j):
                antinote_locs_2.add(loc_delta_2)


print("part 2:", len(antinote_locs_2))
