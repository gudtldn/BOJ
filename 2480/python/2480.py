dice = list(map(int, input().split()))
dice_dict = {k: 0 for k in dice}

for n in dice:
    dice_dict[n] += 1

match len(dice_dict):
    case 1:
        print(10000 + dice[0] * 1000)

    case 2:
        print(1000 + max(dice_dict.items(), key=lambda x: x[1])[0] * 100)

    case 3:
        print(max(dice) * 100)
