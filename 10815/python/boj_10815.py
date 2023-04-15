_ = input()
n_list = sorted(list(map(int, input().split())))
_ = input()
m_list = list(map(int, input().split()))

l = 0
r = len(m_list)-1

for m in m_list:
    l = 0
    r = len(n_list)-1

    while l <= r:
        mid = (l+r)//2

        if m == n_list[mid]:
            print(1, end=" ")
            break

        elif m > n_list[mid]:
            l = mid+1

        else:
            r = mid-1
    else:
        print(0, end=" ")
