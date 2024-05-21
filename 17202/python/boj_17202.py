a = list(input())
b = list(input())

comp = "".join([a[idx] + b[idx] for idx in range(8)])

while len(comp) > 2:
    temp = ""
    for idx in range(len(comp)-1):
        a, b = int(comp[idx]), int(comp[idx+1])
        temp += str((a + b) % 10)
    comp = temp

print(comp)