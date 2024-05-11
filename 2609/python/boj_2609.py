def GCD(x: int, y: int) -> int:
    while y:
        x, y = y, x % y
    return x

def LCM(x: int, y: int) -> int:
    return x * y // GCD(x, y)

a, b = map(int, input().split())
print(GCD(a, b), LCM(a, b), sep="\n")