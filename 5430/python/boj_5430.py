import sys
from collections import deque

_input = sys.stdin.readline
_print = sys.stdout.write

class AC(deque):
    def R(self):
        self.reverse()

    def D(self, rev: bool):
        if self:
            if rev:
                self.pop()
            else:
                self.popleft()
        else:
            return "error"

    def __str__(self):
        return f"[{','.join(map(str, self))}]"

T = int(_input())
for _ in range(T):
    p = _input().rstrip().replace("RR", "")

    n = int(_input())
    if n:
        x = _input().rstrip()[1:-1].split(",")
    else:
        _input()
        x = []

    ac = AC(x)
    rev = False
    for func in p:
        if func == "R":
            rev = not rev
        elif func == "D":
            if ac.D(rev) == "error":
                _print("error\n")
                break
    else:
        if rev:
            ac.R()
        _print(f"{str(ac)}\n")
