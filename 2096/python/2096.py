from sys import stdin
from copy import deepcopy

input = stdin.readline

def list_slice(_list: list, index: int) -> list:
    return _list[0 if index-1 < 0 else index-1:index+2]

n = int(input())

line = list(map(int, input().split()))

line_dict = {
    "min": line[:],
    "max": line[:]
}

for _ in range(n-1):
    line = list(map(int, input().split()))

    temp_dict = deepcopy(line_dict)
    
    for idx in range(len(line)):
        line_slice_min = list_slice(line_dict['min'], idx)
        line_slice_max = list_slice(line_dict['max'], idx)
        
        temp_dict['min'][idx] = min(line_slice_min) + line[idx]
        temp_dict['max'][idx] = max(line_slice_max) + line[idx]
    
    line_dict['min'] = temp_dict['min']
    line_dict['max'] = temp_dict['max']
    

print(max(line_dict['max']), min(line_dict['min']))