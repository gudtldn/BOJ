n, m = map(int, input().split())
cards = list(map(int, input().split()))

result = 0
for i in range(n):
    for j in range(i+1):
        for k in range(j+1):
            if i != j and j != k and k != i:
                total = cards[i] + cards[j] + cards[k]
                result = max(result, total) if total <= m else result

print(result)
