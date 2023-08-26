def quick_sort(arr: list):
    if len(arr) <= 1:
        return arr
    else:
        pivot = arr[0]
        # left = [x for x in arr[1:] if x < pivot]
        left = list(filter(lambda x: x < pivot, arr[1:]))
        print(pivot)
        print(left)
        right = list(filter(lambda x: x >= pivot, arr[1:]))
        return quick_sort(left) + [pivot] + quick_sort(right)


arr = [3, 6, 2, 8, 10, 23, 56, 1]
print(quick_sort(arr))


def test(arr):
    res = []
    for x in arr:
        res.append(x)
    return res

import torch
import torch.optim as optim

opt=optim.Adam()