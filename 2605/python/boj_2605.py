n = int(input())
stack = []

for n, order in enumerate(map(int, input().split()), 1):
    stack.insert(order, n)

print(*reversed(stack))
