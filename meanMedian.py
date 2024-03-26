#!/usr/bin/python3

def mean(n):
    length = len(n)
    total = sum(n)

    return total / length

def median(l):
    length_m = len(l)
    sorted_list = sorted(l)
    mid_index = length_m // 2

    if length_m % 2:
        return sorted_list[mid_index]
    lower, upper = mid_index - 1, mid_index + 1
    return sum(sorted_list[lower:upper]) / 2


# x = [3, 5, 1, 4, 2]
# y = [3, 5, 1, 4, 2, 6, 8, 7]

# print(mean(y)) 
# print(median(y))