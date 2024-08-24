# https://www.acmicpc.net/problem/1373
from typing import Callable

def solution(it_next: Callable[[], str]):
    print(oct(int(it_next(), 2))[2:])

if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
