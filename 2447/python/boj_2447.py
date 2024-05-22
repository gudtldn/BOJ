def star_recur(n):
    if n == 3:
        return ["***", "* *", "***"]
    stars = star_recur(n//3)
    return [x*3 for x in stars] + [x + " "*(n//3) + x for x in stars] + [x*3 for x in stars]

n = int(input()) # n = 3^k, 1 <= k <= 8
print("\n".join(star_recur(n)))
