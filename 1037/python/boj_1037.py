# https://www.acmicpc.net/problem/1037
from typing import Callable

def solution(it_next: Callable[[], str]):
    it_next()
    value = list(map(int, it_next().split()))
    print(max(value) * min(value))

if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
