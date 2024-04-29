n = int(input())
h = list(map(int, input().split()))

sum_h = sum(h)

side_h = 0
for i in range(1, n):
    side_h += abs(h[i-1] - h[i])

side_h += h[0] + h[-1]

print((n + sum_h) * 2 + side_h)