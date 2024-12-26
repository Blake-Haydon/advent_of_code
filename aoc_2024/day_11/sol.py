from functools import lru_cache

@lru_cache(maxsize=None)
def n(i, max_i, stone):
    if i < max_i:
        stone_str = str(stone)
        stone_len = len(stone_str)
        if stone == 0:
            return n(i+1, max_i, 1)
        elif stone_len % 2 == 0:
            return n(i+1, max_i, int(stone_str[:stone_len//2])) + n(i+1, max_i, int(stone_str[stone_len//2:]))
        else:
            return n(i+1, max_i, stone * 2024)

    return 1


stones = [8435, 234, 928434, 14, 0, 7, 92446, 8992692]

total_p1 = 0
for stone in stones:
    total_p1 += n(0, 25, stone)

total_p2 = 0
for stone in stones:
    total_p2 += n(0, 75, stone)

print("part 1:", total_p1)
print("part 2:", total_p2)
