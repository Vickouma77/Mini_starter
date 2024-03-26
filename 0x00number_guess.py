import random

def number_guess():

    print("Welcome to the number guessing game!\n")

    number = random.randrange(1, 10)
    guess = 0
    while guess != number:
        guess = int(input("Enter a number between 1 and 100: "))
        if guess < number:
            print("Too low!")
        elif guess > number:
            print("Too high!")
    print("You got it!")

number_guess()