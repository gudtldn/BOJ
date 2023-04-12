from sys import stdin

class Stack(list):
    def push(self, x: int) -> None:
        super().append(x)
    
    def pop(self) -> int:
        if len(self):
            return super().pop()
        return -1
    
    def size(self) -> int:
        return len(self)
    
    def empty(self) -> int:
        return int(not bool(self))
    
    def top(self) -> int:
        if len(self):
            return self[-1]
        return -1

s = Stack()

out = ""

for _ in range(int(stdin.readline())):
    cmd = stdin.readline()
    
    if cmd.startswith("push"):
        s.push(int(cmd[cmd.index(" "):]))

    elif cmd.startswith("pop"):
        out += str(s.pop()) + "\n"

    elif cmd.startswith("size"):
        out += str(s.size()) + "\n"

    elif cmd.startswith("empty"):
        out += str(s.empty()) + "\n"

    elif cmd.startswith("top"):
        out += str(s.top()) + "\n"

print(out.rstrip())