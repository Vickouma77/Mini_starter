#!/usr/bin/env python3

def only_diff_elements(set_1, set_2):
    """
    function that returns a set of all elements present in only one set
    """

    diff = set_1 ^ set_2

    return diff
