list_a = []
list_b = []
with open("day_01/input.txt", "r") as f:
    for line in f:
        a = int(line.split()[0])
        b = int(line.split()[1])
        list_a.append(a)
        list_b.append(b)

list_a = sorted(list_a)
list_b = sorted(list_b)

list_diff = [abs(list_a[i] - list_b[i]) for i in range(len(list_a))]
print("part 1:", sum(list_diff))

dict_b = dict()
for el in list_b:
    if el in dict_b:
        dict_b[el] += 1
    else:
        dict_b[el] = 1

sim_score = 0
for el in list_a:
    if el in dict_b:
        sim_score += el * dict_b[el]

print("part 2:", sim_score)
