from itertools import combinations

x = input()
y = input()
z = input()
k = int(input())

comb = lambda x: set(combinations(x, k))

x = comb(x)
y = comb(y)
z = comb(z)

result = ["".join(i) for i in sorted(x & y | y & z | z & x)]
print(*result if result else [-1], sep="\n")
