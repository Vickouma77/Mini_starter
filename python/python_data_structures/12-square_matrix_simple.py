#!/usr/bin/env python3

def square_matrix_simple(matrix=[]):
    """function that computes the square value of all integers of a matrix."""

    new_matrix = [list(map(lambda x: x * x, row)) for row in matrix]
    return new_matrix
