(
    a, # 원래의 고기의 온도
    b, # 목표 온도
    c, # 얼어 있는 고기를 1℃ 데우는 데 걸리는 시간
    d, # 얼어 있는 고기를 해동하는 데 걸리는 시간
    e  # 얼어 있지 않은 고기를 1℃ 데우는 데 걸리는 시간 
) = [int(input()) for _ in range(5)]

if a < 0: # 고기가 얼어 있을 때
    time = abs(a) * c + d + (b * e)
else:
    time = (b - a) * e

print(time)