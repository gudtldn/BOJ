t = int(input())
for _ in range(t):
    b = bin(int(input()))[2:][::-1]
    for b_idx in range(len(b)):
        if b[b_idx] == "1":
            print(b_idx, end=" ")