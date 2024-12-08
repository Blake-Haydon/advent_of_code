def extract_middle_num(arr):
    return arr[len(arr) // 2]

def is_correct_order(book):
    correct_order = True
    for i in range(1, len(book)):
        try:
            page = book[i]
            before_pages = set(book[:i])
            potential_before_pages = set(what_is_before[page])
        except:
            correct_order = False
            break

        if not before_pages.issubset(potential_before_pages):
            correct_order = False
            break

    return correct_order, i


count_1 = 0
what_is_before = dict()
books = []
# with open("day_05/toy_input.txt", "r") as f:
with open("day_05/input.txt", "r") as f:
    for line in f:
        if "|" in line:
            before_num = int(line.strip().split("|")[0])
            after_num = int(line.strip().split("|")[1])
            if after_num in what_is_before.keys():
                what_is_before[after_num] = what_is_before[after_num] + [before_num]
            else:
                what_is_before[after_num] = [before_num]

        if "," in line:
            book = [int(num) for num in line.strip().split(",")]
            books.append(book)

incorrect_books = []
for book in books:
    is_correct, _ = is_correct_order(book)
    if is_correct:
        count_1 += extract_middle_num(book)
    else:
        incorrect_books.append(book)

print("part 1:", count_1)



count_2 = 0
for book in incorrect_books:
    for i in range(1, 1000):
        is_correct, i = is_correct_order(book)
        if is_correct:
            count_2 += extract_middle_num(book)
            break

        book[i], book[i-1] = book[i-1], book[i]

print("part 2:", count_2)
