h, w = map(int, input().split())
blocks = list(map(int, input().split()))

# 0: 빈 공간, 1: 블럭, 2: 물
world = [[1 if block-i >= 0 else 2 for block in blocks] for i in range(h, 0, -1)]

for world_idx in range(len(world)):
    for _ in range(2):
        world[world_idx] = world[world_idx][::-1]
        if world[world_idx][0] == 1:
            continue

        elif world[world_idx][0] == 2 and 1 in world[world_idx]:
            world[world_idx][:world[world_idx].index(1)] = [0]*world[world_idx].index(1)

        else:
            world[world_idx] = [0]*len(world[world_idx])

print(sum(w.count(2) for w in world))
