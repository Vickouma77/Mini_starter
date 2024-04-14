#!/usr/bin/python3

def findMissingNumbers(num_list):
    numbers = set(sorted(num_list))
    missing = []

    for i in range(num_list[0], num_list[-1]):
        if i not in numbers:
            missing.append(i)
    return missing

""" list_n = [1, 2, 4, 6, 7, 9]

print(findMissingNumbers(list_n)) """  # [3, 5, 8]