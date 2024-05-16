prime = list(range(5000001))
for i in range(2, int(5000001**0.5)):
    if prime[i] == i:
        for j in range(i*2, 5000001, i):
            if prime[j] == j:
                prime[j] = i

_ = input()
k = map(int, input().split())
for i in k:
    idx = i
    _li = []
    while idx > 1:
        _li.append(prime[idx])
        idx //= prime[idx]
    print(" ".join(map(str, _li)))
