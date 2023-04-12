def get_primes(n):
    primes_li = [True] * (n + 1)
    primes_li[0] = False
    primes_li[1] = False

    for i in range(2, int(n**.5)+1):
        if primes_li[i] == True:
            j = 2

            while i*j <= n:
                primes_li[i*j] = False
                j += 1

    return set(n for n, i in enumerate(primes_li) if i)


primes = get_primes(1000)

_ = input()
li = set(map(int, input().split()))

print(len(primes & li))
