#!/usr/bin/env python3

def fib(n):
    """Print a Fibonacci series up to n."""
    current_fib, next_fib = 0, 1
    for _ in range(0, n):
        fib_number = current_fib
        current_fib, next_fib = next_fib, current_fib + next_fib
        print(fib_number, end=' ')

fib(20)
