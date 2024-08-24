# https://www.acmicpc.net/problem/1032
from typing import Callable

def solution(it_next: Callable[[], str]):
    patterns = [it_next().strip() for _ in range(int(it_next()))]
    for idx in range(len(patterns[0])):
        if all(patterns[0][idx] == pattern[idx] for pattern in patterns[1:]):
            print(patterns[0][idx], end="")
        else:
            print("?", end="")

if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
