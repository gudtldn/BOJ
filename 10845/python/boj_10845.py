import sys

class Queue(list):
    push = list.append

    def pop(self):
        return super().pop(0) if self else -1

    def size(self):
        return len(self)

    def empty(self):
        return int(not self)

    def front(self):
        return self[0] if self else -1

    def back(self):
        return self[-1] if self else -1

queue = Queue()

for _ in range(int(input())):
    cmd, *arg = sys.stdin.readline().strip().split()
    if (result := getattr(queue, cmd)(*arg)) is not None:
        print(result)
