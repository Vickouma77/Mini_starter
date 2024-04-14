from collections import defaultdict

def group_anagrams(words):
    anagrams = defaultdict(list)
    for word in words:
        key = ''.join(sorted(word))
        anagrams[key].append(word)
    return list(anagrams.values())

words = ["eat", "tea", "tan", "ate", "nat", "bat", "tab", "cat", "tac"]
print(group_anagrams(words))