input()
a = set(map(int, input().split()))
b = set(map(int, input().split()))

diff = a - b
print(len(diff))
if diff:
    print(*sorted(diff))
