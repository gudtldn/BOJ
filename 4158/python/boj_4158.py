import sys
_input = sys.stdin.readline

while True:
    n, m = map(int, _input().split())

    if n == 0 and m == 0:
        break

    n_set = set([int(_input()) for _ in range(n)])
    m_set = set([int(_input()) for _ in range(m)])

    print(len(n_set & m_set))
