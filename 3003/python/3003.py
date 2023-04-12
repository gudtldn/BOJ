chess_piece = (1, 1, 2, 2, 2, 8)
pieces = tuple(map(int, input().split()))

for idx in range(len(chess_piece)):
    print(chess_piece[idx] - pieces[idx], end=" ")