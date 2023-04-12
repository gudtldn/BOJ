table = [list(map(int, input().split())) for _ in range(9)]
max_row = [max(row) for row in table]

max_value = max(max_row)
max_row_idx = max_row.index(max_value)

print(max_value, f"{max_row_idx+1} {table[max_row_idx].index(max_value)+1}", sep="\n")
