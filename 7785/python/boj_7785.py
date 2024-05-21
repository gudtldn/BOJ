logs = set()
for _ in range(int(input())):
    name, log = input().split()
    if log == "enter":
        logs.add(name)
    else:
        logs.remove(name)
print(*sorted(logs, reverse=True), sep="\n")