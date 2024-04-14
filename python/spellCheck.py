from spellchecker import SpellChecker

spell = SpellChecker()

word = input("Enter a word:  ")

if word in spell:
    print(f"{word} is spelled correctly...")
else:
    print(f"{word} is spelled incorrectly...\n")

    correct_word = spell.correction(word)
    print(f"Did you mean: {correct_word}?")
    print("\nSuggestions: \n")
    for suggestion in spell.candidates(word):
        print(suggestion)
