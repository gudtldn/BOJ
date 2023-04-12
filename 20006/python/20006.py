p, m = map(int, input().split())
players: list[list[int | str]] = [list(map(lambda x: int(x) if x.isnumeric() else x, input().split())) for _ in range(p)]

rooms: list[list[list[int | str]]] = []
for player_idx in range(p):
    if not rooms:
        rooms.append([players[player_idx]])
        continue

    for room_idx in range(len(rooms)):
        if rooms[room_idx][0][0]-10 <= players[player_idx][0] <= rooms[room_idx][0][0]+10:
            if len(rooms[room_idx]) == m:
                continue
            
            rooms[room_idx].append(players[player_idx])
            break
    else:
        rooms.append([players[player_idx]])

for room in rooms:
    print("Started!\n" if len(room) == m else "Waiting!\n")
    for player in sorted(room, key=lambda x: x[1]):
        print(*player)