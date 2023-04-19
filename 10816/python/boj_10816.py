from collections import defaultdict

_dict = defaultdict(int)

_ = input()

for i in map(int, input().split()):
    _dict[i] += 1

_ = input()
b = map(int, input().split())

print(*[_dict[i] for i in b])
