c = int(input())
test_case = [list(map(int, input().split()[1:])) for _ in range(c)]

for x in test_case:
    print(f"{round(len([i for i in x if sum(x)/len(x) < i])/len(x)*100, 3):.3f}%")
