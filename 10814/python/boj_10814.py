import sys
input = sys.stdin.readline

class User:
    def __init__(self, age: int, name: str, seq: int) -> None:
        self.age = age
        self.name = name
        self.seq = seq

    def __str__(self) -> str:
        return f"{self.age} {self.name}"

def get_split_info(info: str, idx: int):
    age, name = map(lambda x: int(x) if x.isdigit() else x, info.split())
    return User(age, name, idx)


n = int(input())
users = [get_split_info(input().strip(), idx) for idx in range(n)]
print(*sorted(users, key=lambda x: (x.age, x.seq)), sep="\n")