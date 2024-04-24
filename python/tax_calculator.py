#!/usr/bin/env python3

def calculate_tax(income):
    if income <= 10000:
        tax = 0

    elif income >= 10001 and income <= 39999:
        tax = (income * 0.1)
    
    elif income >= 40000 and income <= 89999:
        tax = 700 + (income * 0.2)
    
    else:
        tax = 1000 + (income * 0.3)

    return tax

def main():
    try:
        income = float(input("Enter your income: "))

        tax = calculate_tax(income)

        print(f"Your tax is {tax}")

    except ValueError:
        print("Enter a valid number")
    

if __name__ == '__main__':
    main()
