def is_valid(pos, grid):
    return 0 <= pos[0] < len(grid) and 0 <= pos[1] < len(grid[0])

def find_blob(char, pos, grid, blob_pos):
    """Find all points in a blob of a given character"""
    if not is_valid(pos, grid) or pos in blob_pos:
        return blob_pos

    if grid[pos[0]][pos[1]] == char:
        blob_pos.add(pos)

        up = find_blob(char, (pos[0]-1, pos[1]), grid, blob_pos)
        blob_pos.union(up)

        down = find_blob(char, (pos[0]+1, pos[1]), grid, blob_pos)
        blob_pos.union(down)

        left = find_blob(char, (pos[0], pos[1]-1), grid, blob_pos)
        blob_pos.union(left)

        right = find_blob(char, (pos[0], pos[1]+1), grid, blob_pos)
        blob_pos.union(right)

    return blob_pos

def part_1_score(blob):
    """Calculate the area and perimiter of a blob.
    Return the area * perimiter."""
    blob_perimeter = 0
    for pos in blob:
        if (pos[0]-1, pos[1]) not in blob:
            blob_perimeter += 1
        if (pos[0]+1, pos[1]) not in blob:
            blob_perimeter += 1
        if (pos[0], pos[1]-1) not in blob:
            blob_perimeter += 1
        if (pos[0], pos[1]+1) not in blob:
            blob_perimeter += 1

    blob_area = len(blob)
    return blob_perimeter * blob_area

def part_2_score(blob):
    return -1


grid = []
# with open("day_12/toy_input.txt", "r") as f:
with open("day_12/input.txt", "r") as f:
    for line in f:
        grid.append(line.strip())

visited = set()
blobs = []
for i in range(len(grid[0])):
    for j in range(len(grid)):
        if (i, j) not in visited:
            curr_char = grid[i][j]
            blob_pos = find_blob(curr_char, (i, j), grid, set())

            # add blob to list of blobs to analyze later
            blobs.append(blob_pos)

            # add blob locations to visited
            visited = visited.union(blob_pos)


total_p1 = 0
for blob in blobs:
    total_p1 += part_1_score(blob)
print("part 1:", total_p1)

total_p2 = 0
for blob in blobs:
    total_p2 += part_2_score(blob)
print("part 2:", total_p2)
