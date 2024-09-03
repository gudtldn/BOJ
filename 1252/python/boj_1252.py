# https://www.acmicpc.net/problem/1252
from typing import Callable

def solution(it_next: Callable[[], str]):
    print(bin(sum(map(lambda x: int(x, 2), it_next().split())))[2:])

if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
