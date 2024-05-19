from functools import cache

@cache
def binomial_coefficient(n, k):
    if k == 0 or n == k:
        return 1
    return binomial_coefficient(n-1, k) + binomial_coefficient(n-1, k-1)

print(binomial_coefficient(*map(int, input().split())))
