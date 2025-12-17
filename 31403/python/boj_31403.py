# $A + B - C$
# https://www.acmicpc.net/problem/31403

from typing import Callable

def solution(it_next: Callable[[], str]):
    a, b, c = it_next(), it_next(), int(it_next())
    print(int(a) + int(b) - c)
    print(int(a + b) - c)

if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
