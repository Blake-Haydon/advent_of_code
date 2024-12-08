import re
x = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)) mul( 2 , 4)3"
x = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)"


def parse_to_ints(string):
    assert len(string.split(",")) == 2
    a = string.split(",")[0]
    b = string.split(",")[1]

    for c in a:
        assert c in "1234567890"
    for c in b:
        assert c in "1234567890"

    a, b = int(a), int(b)
    assert a < 1000
    assert b < 1000

    return a, b


def parse_segment(string):
    total = 0
    i = 0
    while i < len(string):
        if string[i:i+4] == "mul(":
            j = i
            while j < len(string) and string[j] != ")":
                j += 1

            # try to parse
            try:
                a, b = parse_to_ints(string[i+4:j])
                total += a * b
            except:
                pass
        i += 1

    return total

def parse_segment_2(string, do_flag):
    total = 0
    i = 0
    while i < len(string):
        if string[i:i+4] == "do()":
            do_flag = True

        if string[i:i+7] == "don't()":
            do_flag = False

        if string[i:i+4] == "mul(":
            j = i
            while j < len(string) and string[j] != ")":
                j += 1

            # try to parse
            try:
                a, b = parse_to_ints(string[i+4:j])
                if do_flag:
                    total += a * b
            except:
                pass

        i += 1

    return total, do_flag



p1_total = 0
with open("day_03/input.txt", "r") as f:
    for line in f:
        p1_total += parse_segment(line)

print("part 1:", p1_total)


p2_total = 0
with open("day_03/input.txt", "r") as f:
    do_flag = True
    for line in f:
        new_total, do_flag = parse_segment_2(line, do_flag)
        p2_total += new_total

# not 85711693
print("part 2:", p2_total)


# generate regex to find "mul(d,d)"

# mul\([0-9]+,[0-9]+\)
#

tot = 0
with open("day_03/input.txt", "r") as f:
    for line in f:
        matches = re.findall(r"mul\([0-9]+,[0-9]+\)", line)
        for match in matches:
            match = match[4:-1]

            a, b = match.split(",")[0], match.split(",")[1]
            a, b = int(a), int(b)

            tot += a * b

        # p1_total += parse_segment(line)

tot2 = 0
with open("day_03/input.txt", "r") as f:
    for line in f:
        for active_line in line.split("do()"):
            active_line = active_line.split("don\'t()")[0]
            matches = re.findall(r"mul\([0-9]+,[0-9]+\)", active_line)

            for match in matches:
                match = match[4:-1]

                a, b = match.split(",")[0], match.split(",")[1]
                a, b = int(a), int(b)

                tot2 += a * b

        # p1_total += parse_segment(line)

print(tot2)
