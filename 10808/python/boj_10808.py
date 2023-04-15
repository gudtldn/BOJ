alphabet = [0] * 26

s = input()

for i in s:
    alphabet[ord(i) - ord("a")] += 1

print(*alphabet)