# https://www.acmicpc.net/problem/1009

from typing import TextIO

def fast_pow(a, x, n):  # a^x mod n
    y = 1
    while x > 0:
        if x & 1 == 1: # 지수의 LSB가 1인지 확인
            y = (a * y) % n
        a = (a * a) % n
        x = x >> 1
    return y

def solution(stdin: TextIO) -> str:
    stdin = iter(stdin.read().split("\n"))
    next = stdin.__next__
    result = []

    for _ in range(int(next())):
        a, b = map(int, next().split())
        result.append(fast_pow(a, b, 10) % 10 or 10)

    return "\n".join(map(str, result))

if __name__ == "__main__":
    print(solution(open(0)))
