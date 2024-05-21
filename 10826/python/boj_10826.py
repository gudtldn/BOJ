import sys
from functools import cache

sys.setrecursionlimit(10**6)

@cache
def Fibo(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    return Fibo(n-1) + Fibo(n-2)

print(Fibo(int(input())))

# n = int(input())
# fibo = [0, 1]
# for i in range(2, n+1):
#     fibo.append(fibo[i-1] + fibo[i-2])
# print(fibo[n])