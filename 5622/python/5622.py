alp = sorted(list("qwertyuiopadfghjklxcvbnm".upper()))

dial: dict[int, tuple[str]] = {
    2: ("A", "B", "C"),
    3: ("D", "E", "F"),
    4: ("G", "H", "I"),
    5: ("J", "K", "L"),
    6: ("M", "N", "O"),
    7: ("S", "P", "Q", "R"),
    8: ("T", "U", "V"),
    9: ("W", "X", "Y", "Z")
}

total_time = 0

for s in input():
    for dial_item in dial.items():
        if s in dial_item[1]:
            total_time += dial_item[0] + 1
            break

print(total_time)