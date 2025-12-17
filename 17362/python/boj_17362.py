# 수학은 체육과목 입니다 2
# https://www.acmicpc.net/problem/17362

from typing import Callable


def solution(it_next: Callable[[], str]):
    n = int(it_next())
    finger = n % 8

    match finger:
        case 1:
            print(1)
        case 2 | 0:
            print(2)
        case 3 | 7:
            print(3)
        case 4 | 6:
            print(4)
        case 5:
            print(5)


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
