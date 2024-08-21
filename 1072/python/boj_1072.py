# https://www.acmicpc.net/problem/10720000

from typing import TextIO

def solution(stdin: TextIO) -> str:
    stdin = iter(stdin.read().split("\n"))
    result = []

    x, y = map(int, next(stdin).split())
    z = (y * 100) // x

    if z >= 99:
        return "-1"

    low, high = 0, 1_000_000_000
    res = 0

    while low <= high:
        mid = (low + high) // 2
        temp_z = ((y + mid) * 100) // (x + mid)

        if z >= temp_z:
            low = mid + 1
            res = low
        else:
            high = mid - 1

    result.append(str(res))
    return "\n".join(result)

if __name__ == "__main__":
    print(solution(open(0)))
