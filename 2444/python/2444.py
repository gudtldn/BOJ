n = int(input())

for i in range(1, 2*n, 2):
    print(("*"*i).center(2*n-1).rstrip())

for i in range(2*n-3, 0, -2):
    print(("*"*i).center(2*n-1).rstrip())
