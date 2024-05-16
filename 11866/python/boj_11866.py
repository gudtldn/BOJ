import sys

input = sys.stdin.readline

n, k = map(int, input().split())
josephus_list = []

rng = list(range(1, n+1))
idx = 0
while rng:
    idx = (idx + k - 1) % len(rng)
    josephus_list.append(rng.pop(idx))

print(f"<{', '.join(map(str, josephus_list))}>")
