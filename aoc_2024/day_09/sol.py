line = ""
with open("day_09/input.txt", "r") as f:
    for line in f:
        line = line.strip()

example_line = "2333133121414131402"
# line = example_line
fs_cnt = 0
fs_output = ""
for i in range(len(line)):
    if i % 2 == 0:
        fs_output += int(line[i]) * str(fs_cnt)
        fs_cnt += 1
    else:
        fs_output += int(line[i]) * "."

i = 0
while "." in fs_output:
    # print(fs_output)
    # bad start
    if fs_output[i] != ".":
        i += 1
        continue

    # bad end
    if fs_output[-1] == ".":
        fs_output = fs_output[:-1]
        continue

    fs_output = fs_output[:i] + fs_output[-1] + fs_output[i+1:-1]

total = 0
for i in range(len(fs_output)):
    total += i * int(fs_output[i])

print(total)
