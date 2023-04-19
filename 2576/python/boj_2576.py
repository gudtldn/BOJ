numbers = [int(input()) for _ in range(7)]
odd = [n for n in numbers if n % 2 == 1]

if len(odd) == 0:
    print(-1)
else:
    print(sum(odd), min(odd), sep="\n")