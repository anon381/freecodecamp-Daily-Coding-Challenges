
import re

def to_camel_case(s):
    # Split by space, dash, or underscore (one or more)
    words = re.split(r'[ _-]+', s)
    # Skip empty strings from splitting
    words = [word for word in words if word]
    
    # First word lowercase, rest capitalized
    if not words:
        return ''
    
    first_word = words[0].lower()
    rest = [word.capitalize() for word in words[1:]]
    
    return first_word + ''.join(rest)


