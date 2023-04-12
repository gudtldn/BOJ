n = int(input())
print(*[("*"*i).rjust(n) for i in range(1, n+1)], sep="\n")