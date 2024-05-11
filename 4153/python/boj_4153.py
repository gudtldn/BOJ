while (case := sorted(map(int, input().split()))) != [0]*3:
    print("right" if case[0] ** 2 + case[1] ** 2 == case[2] ** 2 else "wrong")