# 36진수
# https://www.acmicpc.net/problem/1036

from typing import Callable


def dec_to_base36(value: int) -> str:
    if value == 0:
        return "0"

    chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"

    res = ""
    while value > 0:
        res = chars[value % 36] + res
        value //= 36

    return res


def base36_to_dec(value: str) -> int:
    return int(value, 36)


def solution(it_next: Callable[[], str]):
    n = int(it_next())
    raw_words = [it_next() for _ in range(n)]
    k = int(it_next())

    total_base36_sum = 0
    profits = [0] * 36
    for word in raw_words:
        total_base36_sum += base36_to_dec(word)

        # 각 문자의 자릿수에 따른 이익 계산
        for i, char in enumerate(reversed(word)):
            # 해당 문자가 Z로 변했을 때의 차이값 * 자릿수 가중치
            val = base36_to_dec(char)
            profits[val] += (35 - val) * (36**i)

    # 이익이 큰 순서대로 정렬
    profits.sort(reverse=True)

    # 총합 = 원래 합 + 상위 k개의 이익
    sum_all = total_base36_sum + sum(profits[:k])
    print(dec_to_base36(sum_all))


if __name__ == "__main__":
    solution(iter(open(0).read().splitlines()).__next__)
