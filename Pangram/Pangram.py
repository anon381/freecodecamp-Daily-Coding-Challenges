import string

def is_pangram(sentence, letters):
    sentence = sentence.lower()
    letters = set(letters.lower())

    filtered = {ch for ch in sentence if ch.isalpha()}

    return filtered == letters
