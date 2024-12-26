def load_circuit():
    pass

def run_circuit():
    pass

class Circuit:
    pass

wires = dict()
seen_wires = set()
ops_and = []
ops_or = []
ops_xor = []
with open("day_24/input.txt", "r") as f:
# with open("day_24/toy_input.txt", "r") as f:
    for line in f:
        line = line.strip()
        split_input = line.split(": ")
        if len(split_input) == 2:
            # input wire values
            wire_label = split_input[0]
            wire_value = int(split_input[1])

            # assign wire value
            wires[wire_label] = wire_value
        else:
            # input wire operations
            split_inner = line.split(" -> ")
            and_line = split_inner[0].split(" AND ") + [split_inner[-1]]
            or_line = split_inner[0].split(" OR ") + [split_inner[-1]]
            xor_line = split_inner[0].split(" XOR ") + [split_inner[-1]]

            if len(and_line) == 3:
                ops_and.append(and_line)
                seen_wires.add(and_line[0])
                seen_wires.add(and_line[1])
                seen_wires.add(and_line[2])
            if len(or_line) == 3:
                ops_or.append(or_line)
                seen_wires.add(or_line[0])
                seen_wires.add(or_line[1])
                seen_wires.add(or_line[2])
            if len(xor_line) == 3:
                ops_xor.append(xor_line)
                seen_wires.add(xor_line[0])
                seen_wires.add(xor_line[1])
                seen_wires.add(xor_line[2])



# run the operations
while len(ops_and) > 0 or len(ops_or) > 0 or len(ops_xor) > 0:
    for op in ops_and:
        if op[0] in wires and op[1] in wires:
            wires[op[2]] = wires[op[0]] & wires[op[1]]
            ops_and.remove(op)

    for op in ops_or:
        if op[0] in wires and op[1] in wires:
            wires[op[2]] = wires[op[0]] | wires[op[1]]
            ops_or.remove(op)

    for op in ops_xor:
        if op[0] in wires and op[1] in wires:
            wires[op[2]] = wires[op[0]] ^ wires[op[1]]
            ops_xor.remove(op)

# for i in range(45, -1, -1):
#     wire_label = f"z{i:02d}"
#     print(f"{wire_label}: {wires[wire_label]}")

output_bin = ""
for i in range(45, -1, -1):
    wire_label = f"z{i:02d}"
    output_bin += str(wires[wire_label])

print(f"part 1: {int(output_bin, 2)}: {output_bin}")

# part 2

print(len(seen_wires))
