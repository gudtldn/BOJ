# 집합
# https://www.acmicpc.net/problem/11723


# 메모리 초과되서 안됨
from typing import Callable


class BitSet:
    def __init__(self):
        self.value: int = 0

    @staticmethod
    def to_bit(in_value: str) -> int:
        return 1 << int(in_value) - 1

    def add(self, in_value: str):
        self.value |= BitSet.to_bit(in_value)

    def remove(self, in_value: str):
        self.value &= ~BitSet.to_bit(in_value)

    def check(self, in_value: str):
        print(int((self.value & BitSet.to_bit(in_value)) != 0))

    def toggle(self, in_value: str):
        self.value ^= BitSet.to_bit(in_value)

    def all(self):
        self.value = (1 << 20) - 1

    def empty(self):
        self.value = 0


def solution(it_next: Callable[[], str]):
    my_set = BitSet()

    n = int(it_next())
    for _ in range(n):
        cmd, *args = it_next().split()
        getattr(my_set, cmd)(*args)


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
