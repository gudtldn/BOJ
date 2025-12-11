# 이상한 기호
# https://www.acmicpc.net/problem/15964

from typing import Callable


class 숫자:
    def __init__(self, in_value: str):
        self.value: int = int(in_value)

    def __matmul__(self, other: "숫자") -> "숫자":
        a = self.value
        b = other.value
        return (a + b) * (a - b)

    def __str__(self) -> str:
        return str(self.value)


def solution(it_next: Callable[[], str]):
    a, b = map(lambda x: 숫자(x), it_next().split())
    print(a @ b)


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
