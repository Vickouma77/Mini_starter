#!usr/bin/python3

def divisible_by_2(my_list=[]):
    multiple_two = []
    for i in range(len(my_list)):
        if my_list[i] % 2 == 0:
            multiple_two.append(True)
        else: 
            multiple_two.append(False)
    return (multiple_two)
