# Electric Bill
# https://www.acmicpc.net/problem/25881

from typing import Callable


def solution(it_next: Callable[[], str]):
    rate, additional = map(int, it_next().split())
    n = int(it_next())

    for _ in range(n):
        usage = int(it_next())
        bill = min(usage, 1000) * rate
        if usage > 1000:
            bill += (usage - 1000) * additional
        print(f"{usage} {bill}")


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
