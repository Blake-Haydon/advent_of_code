import numpy as np
from itertools import combinations

def letter_to_int(letter):
    """converts a letter to an integer"""
    return ord(letter) - ord('a')

def two_string_to_int(string):
    """converts a two letter string to an integer"""
    return letter_to_int(string[0]) * 26 + letter_to_int(string[1])


table = np.zeros((26*26, 26*26), dtype=bool)
seen = set()    # all nodes
seen_t = set()  # nodes that start with t
# with open("day_23/toy_input.txt", "r") as f:
with open("day_23/input.txt", "r") as f:
    for line in f:
        a_chars = line.strip().split("-")[0]
        b_chars = line.strip().split("-")[1]

        a = two_string_to_int(a_chars)
        b = two_string_to_int(b_chars)

        table[a, b] = True
        table[b, a] = True

        # check if the number starts with the char 't'
        if a_chars[0] == 't':
            seen_t.add(a)
        if b_chars[0] == 't':
            seen_t.add(b)

        seen.add(a)
        seen.add(b)


cnt_p1 = 0
for (a, b) in combinations(seen, 2):
    for t in seen_t:
        # check that a dn b are not the t node
        if a == t or b == t:
            break

        # check if a, b, and t are all connected
        if table[a, t] and table[b, t] and table[a, b]:
            cnt_p1 += 1

print("part 1:", cnt_p1)

for i in range(26*26):
    print(np.sum(table[i]))
