#!/usr/bin/python3

def replace_in_list(my_list, idx, element):
    """
    If idx is negative, the function should not modify anything, 
    and returns the original list

    If idx is out of range (> of number of element in my_list), 
    the function should not modify anything, 
    and returns the original list
    """
    if idx >= 0 and idx < len(my_list):
        my_list[idx] = element
    return (my_list)
