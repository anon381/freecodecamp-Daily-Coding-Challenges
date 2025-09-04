
def repeat_vowels(s):
    vowels = "aeiouAEIOU"
    result = []
    count = 0  

    for char in s:
        if char in vowels:
            count += 1
            
            result.append(char + char.lower() * (count - 1))
        else:
            result.append(char)
    
    return ''.join(result)


