seq = [j for i in range(1, 46) for j in [i]*i]
a, b = map(int, input().split())
print(sum(seq[a-1:b]))