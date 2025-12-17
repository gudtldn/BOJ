# 지속
# https://www.acmicpc.net/problem/11648

from math import prod
from typing import Callable


def solution(it_next: Callable[[], str]):
    iter_num = 0
    n = it_next()
    while len(n) > 1:
        iter_num += 1
        n = str(prod(map(int, n)))

    print(iter_num)


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
