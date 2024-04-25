#!/usr/bin/env python3

def search_replace(my_list, search, replace):
    """
    replaces all occurrences of an element by another in a new list.

    Args:
    my_list: list of elements.
    search: element to search in the list.
    replace: element to replace the search element.

    Returns:
        new_list: list with the replaced elements.
    """
    new_list = [replace if x == search else x for x in my_list]
    return new_list
