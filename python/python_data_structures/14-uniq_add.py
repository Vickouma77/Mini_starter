#!/usr/bin/env python3

def uniq_add(my_list=[]):
    """ 
    function that adds all unique integers in a list (only once for each integer)
    """
    new_list = set(my_list)

    return sum(new_list)
