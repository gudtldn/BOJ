n = int(input())
student = {name: 0 for name in input().split()}

for i in range(n):
    for name in input().split():
        student[name] += 1

for info in sorted(student.items(), key=lambda x: (-x[1], x[0])):
    print(*info)
