for _ in range(int(input())):
    total = 0
    x = 0
    for ox in input():
        x += 1
        match ox:
            case "O":
                total += x
            case "X":
                x = 0
    print(total)