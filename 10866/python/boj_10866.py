import sys

class Deque(list):
    def push_front(self, obj):
        self.insert(0, obj)

    def push_back(self, obj):
        self.append(obj)

    def pop_front(self):
        if self.empty():
            return -1
        return self.pop(0)

    def pop_back(self):
        if self.empty():
            return -1
        return self.pop()

    def size(self):
        return len(self)

    def empty(self):
        return int(not self)

    def front(self):
        if self.empty():
            return -1
        return self[0]

    def back(self):
        if self.empty():
            return -1
        return self[-1]

deque = Deque()
for _ in range(int(input())):
    cmd, *arg = sys.stdin.readline().strip().split()

    if (result := getattr(deque, cmd)(*arg)) is not None:
        print(result)
